(ns www.core
  (:require-macros [cljs.core.async.macros :refer [go]])
  (:require
   [cljs-http.client :as http]
   [cljs.core.async :refer [<!]]
   [reagent.core :as r]))

;; -------------------------
;; Data
(defonce items (r/atom []))

;; -------------------------
;; Views

(defn nav-panel []
  (let [current-search (r/atom "")]
    (fn []
      [:div {:class "SideNav"}
       [:div {:class "input"}
        [:input {:type "text"
                 :value @current-search
                 :on-change #(reset! current-search (-> % .-target .-value))}]
        [:i {:class (if (clojure.string/blank? @current-search) "" "active")
             :on-click #(reset! current-search "")} "×"]]])))

(defn art-view []
  [:div {:class "ArtView"}])

(defn track-row [track]
  [:tr {:key (:id track) :class "TrackEntry"}
   [:td (:title track)]
   [:td (:artist track)]
   [:td (:album track)]
   [:td (:year track)]])

(defn track-table []
  [:div {:class "TrackList"}
   [:table
    [:thead
     [:tr
      [:th "Title"]
      [:th "Artist"]
      [:th "Album"]
      [:th "Year"]]]
    [:tbody (for [item @items]
          (track-row item))]]])

(defn home-page []
  [:div  {:class "SplitPane"}
   [nav-panel]
   [:div {:class "Collection"}
    [art-view]
    [:div {:class "PaneDivider"}]
    [track-table]]])

;; -------------------------
;; Initialize app

(defn mount-root []
  (r/render [home-page] (.getElementById js/document "app")))

(defn init! []
  (go (let [response (<! (http/get "item"))]
        (reset! items (:body response))))
  (mount-root))
