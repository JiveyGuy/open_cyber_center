const axios = require('axios');
const querystring = require('querystring');
const apiUrl = 'https://pixabay.com/api';
const apikey = 'Pixabay api key';
export const photosMixin = {
    methods: {
      getPhotos(page = 1) {
            const params = {
                page,
                key: apikey,
                per_page: 21
            }
            const queryString = querystring.stringify(params);
            return axios.get(`${apiUrl}/?${queryString}`);
        },
      searchPhoto(data) {
            let params = Object.assign({}, data);
            params['key'] = apikey;
            params['per_page'] = 21;
            Object.keys(params).forEach(key => {
                if (!params[key]) {
                    delete params[key];
                }
            })
            const queryString = querystring.stringify(params);
            return axios.get(`${apiUrl}/?${queryString}`);
        },
      searchVideo(data) {
            let params = Object.assign({}, data);
            params['key'] = apikey;
            params['per_page'] = 21;
            Object.keys(params).forEach(key => {
                if (!params[key]) {
                    delete params[key];
                }
            })
            const queryString = querystring.stringify(params);
            return axios.get(`${apiUrl}/videos/?${queryString}`);
        }
    }
}