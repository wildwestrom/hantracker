(ns kanji-progress-tracker.core
  (:require [clojure.edn :as edn]
            [clojure.java.io :as io]
            [clojure.set :as set]
            [kanji-progress-tracker.kanjidic2 :refer [init-kanjidic-file]]
            [datahike.api :as d]))

;; Use this if the kanjidic and joyo files don't exist.
#_(init-kanjidic-file)

(def kanjidic (edn/read-string (slurp (io/resource "kanjidic2.edn"))))

(def joyo-kanji (set (edn/read-string (slurp (io/resource "joyo.edn")))))

(defn filter-kanji-set
  "Takes a set of kanji characters as strings and returns a subset of kanjidic."
  [kanji-set]
  (filter (fn [x]
            (set/subset? (set (list (:literal x)))
                         kanji-set))
          kanjidic))

(def kanjidic-joyo-only (filter-kanji-set joyo-kanji))

(def kanjidic-jlpt-only (filter :jlpt kanjidic))

(def cfg {:store {:backend :file :path "/tmp/kanji-progress-testing"}})

(d/create-database cfg)

(def conn (d/connect cfg))

(defn store-all [kanji-list db]
  ())
