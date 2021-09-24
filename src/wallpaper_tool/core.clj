(ns wallpaper-tool.core
  (:require [clojure.java.io :as io]
            [seesaw.bind :as b]
            [seesaw.color :as c]
            [seesaw.core :as s]
            [seesaw.graphics :as g])
  (:import java.awt.Dimension
           java.awt.image.BufferedImage
           javax.imageio.ImageIO))

;; Prod env
(def env
  (atom {:on-close :exit}))

;; Dev env
#_(swap! env assoc :on-close :dispose)

(def wallpaper-width (atom 200))
(def wallpaper-height (atom 200))

(defn paint-canvas [c g]
  (let [w2 (/ @wallpaper-width 2)
        h2 (/ @wallpaper-height 2)
        min-rad (min w2 h2)
        scale-factor (min (/ (.getWidth c) @wallpaper-width)
                          (/ (.getHeight c) @wallpaper-height))]
    (doto g
      (g/anti-alias)
      (g/scale scale-factor)
      (g/draw
       (g/rect 0 0 @wallpaper-width @wallpaper-height)
       (g/style :background (c/color "white")))
      (g/draw
       (g/circle w2 h2 (* 0.8 min-rad))
       (g/style :background (c/color "black"))))))

(defn paint-wallpaper [w h]
  (let [bufimg (g/buffered-image w h BufferedImage/TYPE_INT_RGB)
        g (.getGraphics bufimg)]
    (paint-canvas (Dimension. w h) g)
    bufimg))

(defn make-wallpaper
  [width height]
  (try
    (ImageIO/write (paint-wallpaper width height)
                   "png" (io/file "./resources/wallpaper.png"))
    (catch java.io.IOException e (println e))))

(defn number-input-text
  [atom]
  (let [textfield (s/text :text @atom)]
    (b/bind
     textfield
     (b/transform #(try (reset! atom (Long/parseLong %))
                        (catch NumberFormatException e
                          (println "Not a number" %))
                        (catch Exception e
                          (println "Error:" e))))
     (b/bind
      (b/transform #(if (int? %)
                      "white" "lightcoral"))
      (b/property textfield :background)))
    textfield))

(defn main-panel
  []
  (s/border-panel
   :center (s/canvas :id :canvas :background (c/color "white") :paint paint-canvas)
   :south (s/horizontal-panel
           :items [(s/action :name "Create Wallpaper"
                             :handler (fn [_] (try (make-wallpaper @wallpaper-width @wallpaper-height)
                                                   (catch Exception e (.printStackTrace e)))))
                   (s/label "Width:")
                   (number-input-text wallpaper-width)
                   (s/label "Height:")
                   (number-input-text wallpaper-height)])))

#_(require 'seesaw.dev)

#_(seesaw.dev/show-events (s/text))

(defn load-gui
  []
  (s/invoke-later
   (-> (s/frame :title "simple circle"
                :content (main-panel)
                :minimum-size [640 :by 400]
                :on-close (:on-close @env))
       s/pack!
       s/show!)))
