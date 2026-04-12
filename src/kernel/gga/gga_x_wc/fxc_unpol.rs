//! GGA_X_WC fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_wc.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_wc_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = M_CBRT2;
        let t27 = t26 * t26;
        let t28 = sigma[ip] * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t34 = t25 * t28 * t32;
        let t36 = t25 * sigma[ip];
        let t37 = t27 * t32;
        let t39 = f64::exp(-t34 / 24.0);
        let t40 = t37 * t39;
        let t43 = t20 * t20;
        let t46 = t43 / t22 / t21;
        let t47 = sigma[ip] * sigma[ip];
        let t49 = t29 * t29;
        let t50 = t49 * rho[ip];
        let t52 = 1.0 / t18 / t50;
        let t56 = 1.0 + 0.27560657413756315278e-4 * t46 * t47 * t26 * t52;
        let t57 = f64::ln(t56);
        let t58 = 0.804e0 + 5.0 / 972.0 * t34 + 0.4002424276710846245e-2 * t36 * t40 + t57;
        let t61 = 0.1804e1 - 0.646416e0 / t58;
        let t65 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t17 * t18 * t61);
        let tzk0 = 2.0 * t65;
        zk[ip] += tzk0;
        let t66 = 1.0 / t30;
        let t71 = t3 * t17;
        let t72 = t58 * t58;
        let t73 = 1.0 / t72;
        let t74 = t18 * t73;
        let t75 = t29 * rho[ip];
        let t77 = 1.0 / t30 / t75;
        let t81 = t27 * t77;
        let t82 = t81 * t39;
        let t85 = t46 * t47;
        let t86 = t49 * t29;
        let t88 = 1.0 / t18 / t86;
        let t89 = t26 * t88;
        let t90 = t89 * t39;
        let t93 = 1.0 / t56;
        let t94 = t89 * t93;
        let t97 = -10.0 / 729.0 * t25 * t28 * t77 - 0.10673131404562256653e-1 * t36 * t82 + 0.88942761704685472111e-3 * t85 * t90 - 0.14699017287336701482e-3 * t85 * t94;
        let t102 = piecewise3(t2, 0.0, -t6 * t17 * t66 * t61 / 8.0 - 0.16551095363746320496e0 * t71 * t74 * t97);
        let tvrho0 = 2.0 * rho[ip] * t102 + 2.0 * t65;
        vrho[ip] += tvrho0;
        let t109 = t46 * sigma[ip];
        let t110 = t26 * t52;
        let t111 = t110 * t39;
        let t114 = t110 * t93;
        let t117 = 5.0 / 972.0 * t25 * t37 + 0.4002424276710846245e-2 * t25 * t40 - 0.33353535639257052042e-3 * t109 * t111 + 0.55121314827512630556e-4 * t109 * t114;
        let t121 = piecewise3(t2, 0.0, -0.16551095363746320496e0 * t71 * t74 * t117);
        let tvsigma0 = 2.0 * rho[ip] * t121;
        vsigma[ip] += tvsigma0;
        let t125 = 1.0 / t30 / rho[ip];
        let t130 = t66 * t73;
        let t135 = 1.0 / t72 / t58;
        let t136 = t18 * t135;
        let t137 = t97 * t97;
        let t142 = 1.0 / t30 / t49;
        let t146 = t27 * t142;
        let t147 = t146 * t39;
        let t150 = t49 * t75;
        let t152 = 1.0 / t18 / t150;
        let t153 = t26 * t152;
        let t154 = t153 * t39;
        let t157 = t21 * t21;
        let t158 = 1.0 / t157;
        let t159 = t47 * sigma[ip];
        let t160 = t158 * t159;
        let t161 = t49 * t49;
        let t162 = t161 * t29;
        let t163 = 1.0 / t162;
        let t167 = t153 * t93;
        let t172 = t20 / t23 / t157;
        let t173 = t47 * t47;
        let t174 = t172 * t173;
        let t175 = t161 * t49;
        let t177 = 1.0 / t30 / t175;
        let t179 = t56 * t56;
        let t180 = 1.0 / t179;
        let t181 = t27 * t177 * t180;
        let t184 = 110.0 / 2187.0 * t25 * t28 * t142 + 0.39134815150061607728e-1 * t36 * t147 - 0.800484855342169249e-2 * t85 * t154 + 0.11859034893958062948e-2 * t160 * t163 * t39 + 0.93093776153132442719e-3 * t85 * t167 - 0.12963666552805392131e-6 * t174 * t181;
        let t189 = piecewise3(t2, 0.0, t6 * t17 * t125 * t61 / 12.0 - 0.11034063575830880331e0 * t71 * t130 * t97 + 0.33102190727492640992e0 * t71 * t136 * t137 - 0.16551095363746320496e0 * t71 * t74 * t184);
        let tv2rho20 = 2.0 * rho[ip] * t189 + 4.0 * t102;
        v2rho2[ip] += tv2rho20;
        let t195 = t71 * t18;
        let t196 = t135 * t117;
        let t197 = t196 * t97;
        let t204 = t46 * t26;
        let t209 = t158 * t47;
        let t210 = t161 * rho[ip];
        let t211 = 1.0 / t210;
        let t217 = t172 * t159;
        let t218 = t161 * t75;
        let t220 = 1.0 / t30 / t218;
        let t225 = -10.0 / 729.0 * t25 * t81 - 0.10673131404562256653e-1 * t25 * t82 + 0.26682828511405641633e-2 * t204 * t88 * sigma[ip] * t39 - 0.44471380852342736056e-3 * t209 * t211 * t39 - 0.29398034574673402963e-3 * t109 * t94 + 0.48613749573020220489e-7 * t217 * t27 * t220 * t180;
        let t230 = piecewise3(t2, 0.0, -0.55170317879154401653e-1 * t71 * t130 * t117 + 0.33102190727492640992e0 * t195 * t197 - 0.16551095363746320496e0 * t71 * t74 * t225);
        let tv2rhosigma0 = 2.0 * rho[ip] * t230 + 2.0 * t121;
        v2rhosigma[ip] += tv2rhosigma0;
        let t233 = t117 * t117;
        let t239 = t158 * sigma[ip];
        let t240 = 1.0 / t161;
        let t248 = 1.0 / t30 / t162;
        let t250 = t27 * t248 * t180;
        let t253 = -0.66707071278514104084e-3 * t46 * t111 + 0.16676767819628526021e-3 * t239 * t240 * t39 + 0.55121314827512630556e-4 * t46 * t114 - 0.18230156089882582683e-7 * t172 * t47 * t250;
        let t258 = piecewise3(t2, 0.0, 0.33102190727492640992e0 * t71 * t136 * t233 - 0.16551095363746320496e0 * t71 * t74 * t253);
        let tv2sigma20 = 2.0 * rho[ip] * t258;
        v2sigma2[ip] += tv2sigma20;
    }
}
