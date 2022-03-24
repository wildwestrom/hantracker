(ns kanji-progress-tracker.kanjidic2
  (:require [clojure.data.xml :as dx]
            [clojure.java.io :as io]
            [clojure.pprint :as pp]
            [xml-in.core :as xml]))

(def kanji-xml-data
  (-> "kanjidic2.xml"
      io/resource
      slurp
      java.io.StringReader.
      dx/parse))

(def raw-dict
  (xml/find-first kanji-xml-data [:kanjidic2]))

(def kanjis
  (map :content
       (filter #(and (not= :header (:tag %))
                     (map? %))
               raw-dict)))

(defn elem->maps
  [element {:keys [attr-key attr-lookup val-key]
            :or {val-key :val}}]
  (mapv (fn [x]
          (cond-> {val-key (first (:content x))}
            (and attr-key attr-lookup)
            (assoc attr-key (attr-lookup (:attrs x)))))
        (vec element)))

(defn tag-matches? [key1 tag]
  (= key1 (:tag tag)))

(defn kanji-data [kanji]
  (into
   {}
   (filter
    #(seq (val %))
    {:literal      (first (xml/find-first kanji [:literal]))
     :codepoints   (elem->maps (xml/find-first kanji [:codepoint])
                               {:attr-lookup :cp_type
                                :attr-key    :type
                                :val-key     :codepoint})
     :radicals     (elem->maps (xml/find-first kanji [:radical])
                               {:attr-lookup :rad_type
                                :attr-key    :type
                                :val-key     :radical})
     :grade        (first (xml/find-first kanji [:misc :grade]))
     :stroke-count (first (xml/find-first kanji [:misc :stroke_count]))
     :variants     (elem->maps (filter #(tag-matches? :variant %)
                                       (xml/find-all kanji [:misc]))
                               {:val-key     :variant
                                :attr-lookup :var_type
                                :attr-key    :type})
     :frequency    (first (xml/find-first kanji [:misc :freq]))
     :jlpt         (first (xml/find-first kanji [:misc :jlpt]))
     :dict-numbers (elem->maps  (xml/find-first kanji [:dic_number])
                                {:val-key     :ref-number
                                 :attr-lookup :dr_type
                                 :attr-key    :type})
     :query-codes  (elem->maps  (xml/find-first kanji [:query_code])
                                {:val-key     :q-code
                                 :attr-lookup :qc_type
                                 :attr-key    :type})
     :readings     (concat (elem->maps (filter #(tag-matches? :reading %)
                                               (xml/find-all kanji [:reading_meaning :rmgroup]))
                                       {:val-key     :reading
                                        :attr-lookup :r_type
                                        :attr-key    :type})
                           (mapv (fn [x] {:reading x :type "ja_nanori"})
                                 (xml/find-all kanji [:reading_meaning :nanori])))
     :meanings     (map #(if (nil? (:lang %)) (assoc % :lang "en") %)
                        (elem->maps (filter #(tag-matches? :meaning %)
                                            (xml/find-all kanji [:reading_meaning :rmgroup]))
                                    {:val-key     :meaning
                                     :attr-lookup :m_lang
                                     :attr-key    :lang}))})))

(def all-kanji-data (mapv kanji-data kanjis))

(defn kanji-xml->edn []
  (with-out-str
    (pp/pprint all-kanji-data)))

(defn init-kanjidic-file []
  (spit (io/file "resources/kanjidic2.edn")
        (kanji-xml->edn)))
