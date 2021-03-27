import Vue from "vue";
import Vuex from "vuex";
import numberPlace from './modules/number_place'

Vue.use(Vuex);

export default new Vuex.Store({
  modules: {
    numberPlace,
  }
});
