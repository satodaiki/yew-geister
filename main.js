import './static/scss/main.scss';

import("./pkg").then(module => {
  module.run_app();
});
