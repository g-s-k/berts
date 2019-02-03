(ns www.core
  (:require
   ;; [cljs.core.async :refer [<!]] [cljs-http.client :as http]
   [reagent.core :as r])
  ;; (:require-macros [cljs.core.async.macros :refer [go]])
  )

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

(defn track-table []
  [:div {:class "TrackList"}])

(defn home-page []
  [:div  {:class "SplitPane"}
   [nav-panel]
   [:div {:class "Collection"}
    [art-view]
    [track-table]]])

;; -------------------------
;; Initialize app

(defn mount-root []
  (r/render [home-page] (.getElementById js/document "app")))

(defn init! []
  ;; (go (let [response (<! (http/get "item"))]
  ;;       (prn (:status response))
  ;;       (prn (:body response))))
  (mount-root))
