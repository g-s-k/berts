(ns ^:figwheel-no-load www.dev
  (:require
    [www.core :as core]
    [devtools.core :as devtools]))


(enable-console-print!)

(devtools/install!)

(core/init!)
