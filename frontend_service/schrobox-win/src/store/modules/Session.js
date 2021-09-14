import ProfileAPI from "@/assets/api_wrapper/ProfileAPI.js";

const profileAPI = new ProfileAPI();

export const state = {
  accessToken: null,
  accessExpire: null,
  refreshToken: null,
  refreshExpire: null,
  userId: null,
  firstName: "",
  lastName: "",
};

// commit
export const mutations = {
  setAccessToken(state, [token, expire]) {
    state.accessToken = token;
    state.accessExpire = expire;
  },
  setRefreshToken(state, [token, expire]) {
    state.refreshToken = token;
    state.refreshExpire = expire;
    let expireTimeStamp = Date.now() + expire * 1000;
    localStorage.setItem("refreshToken", token);
    localStorage.setItem("refreshExpireTime", expireTimeStamp);
    localStorage.setItem("refreshExpire", expire);
  },
  setUserId(state, id) {
    state.userId = id;
  },
  clear(state) {
    state.accessToken = null;
    state.accessExpire = null;
    state.refreshToken = null;
    state.refreshExpire = null;
    state.userId = null;
  },
};

export const getters = {
  isLoggedIn(state) {
    return state.accessToken !== null;
  },
  isLocalTokenValid() {
    let expiringTime = localStorage.getItem("refreshExpireTime");
    let refreshExpire = localStorage.getItem("refreshExpire");
    let refreshToken = localStorage.getItem("refreshToken");
    if (
      expiringTime !== null &&
      refreshToken !== null &&
      refreshExpire !== null
    ) {
      if (Date.now() <= expiringTime - 10000) {
        return true;
      }
    }
    return false;
  },
  getLocalToken() {
    return localStorage.getItem("refreshToken");
  },
  getLocalExpire() {
    return localStorage.getItem("refreshExpire");
  },
};

// dispatch
export const actions = {
  async login(context, { username, password }) {
        let response = await profileAPI.login(username, password);
        context.commit("setAccessToken", [
          response.accessToken,
          response.accessExpire,
        ]);
        context.commit("setRefreshToken", [
          response.refreshToken,
          response.refreshExpire,
        ]);
        context.commit("setUserId", response.userId);
        context.dispatch("watchToken");
  },
  logout(context) {
    localStorage.removeItem("refreshToken");
    localStorage.removeItem("refreshExpire");
    localStorage.removeItem("refreshExpireTime");

    context.commit("clear");
  },
  signup(context, { email, password, firstName, lastName }) {
    return new Promise((resolve, reject) => {
      profileAPI.signup(email, password, firstName, lastName).then(
        (response) => {
          resolve(response);
        },
        (err) => {
          reject(err);
        }
      );
    });
  },
};

export default {
  state,
  mutations,
  actions,
  getters,
};
