(ns wallpaper-tool.core
  (:require [clojure.java.io :as io]
            [seesaw.bind :as b]
            [seesaw.core :as s]
            [seesaw.dev :as dev]
            [seesaw.graphics :as g])
  (:import (java.awt Canvas Graphics2D Dimension)
           (java.awt.image BufferedImage)
           (javax.imageio ImageIO)))

(set! *warn-on-reflection* true)

(def wallpaper-width (atom 2880))
(def wallpaper-height (atom 1800))

(defn paint-g2d [c ^Graphics2D g]
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
       (g/style :background "white"))
      (g/draw
       (g/circle w2 h2 (* 0.8 min-rad))
       (g/style :background "black")))))

(defn paint-to-image ^BufferedImage [w h]
  (let [bufimg ^BufferedImage (g/buffered-image w h BufferedImage/TYPE_INT_RGB)
        g (.getGraphics bufimg)]
    (paint-g2d (Dimension. w h) g)
    bufimg))

(defn make-wallpaper [w h]
  (try
    (ImageIO/write (paint-to-image w h)
                   "png" (io/file "./resources/wallpaper.png"))
    (catch java.io.IOException e (println e))))

(defn number-input-text [atom]
  (let [textfield (s/text :text @atom
                          :id :num-input)]
    (b/bind
     textfield
     (b/transform #(try (reset! atom (Long/parseLong %))
                        (catch NumberFormatException _
                          (println "Not a number" %))
                        (catch Exception e
                          (println "Error:" e))))
     (b/bind
      (b/transform #(if (int? %)
                      "white" "lightcoral"))
      (b/property textfield :background)))
    textfield))

(defn main-panel []
  (s/border-panel
   :center (s/canvas :id :main-canvas :paint paint-g2d)
   :south (s/flow-panel
           :items [(s/horizontal-panel
                    :items [(s/label "Width:")
                            (number-input-text wallpaper-width)
                            (s/label "Height:")
                            (number-input-text wallpaper-height)])
                   (s/label :text "Generate Wallpaper?"
                            :h-text-position :right
                            :v-text-position :bottom)])))

(defn load-gui []
  (s/invoke-later
   (-> (s/dialog :title "Wallpaper Preview"
                 :id :main-frame
                 :type :plain
                 :option-type :ok-cancel
                 :success-fn (fn [_] (try (make-wallpaper @wallpaper-width @wallpaper-height)
                                          (catch Exception e (.printStackTrace e))))
                 :default-option :ok
                 :content (main-panel)
                 :minimum-size [640 :by 400]
                 :on-close :dispose)
       s/pack!
       s/show!)))

(load-gui)
