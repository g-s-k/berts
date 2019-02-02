(ns www.core
  (:require
   [reagent.core :as r]))

;; -------------------------
;; Views

(defn home-page []
  [:div  {:class "SplitPane"}
   [:div {:class "SideNav"}]
   [:div {:class "Collection"}
    [:div {:class "ArtView"}]
    [:div {:class "TrackList"}]]])

;; -------------------------
;; Initialize app

(defn mount-root []
  (r/render [home-page] (.getElementById js/document "app")))

(defn init! []
  (mount-root))
