//! GGA_X_SG4 fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sg4.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_sg4_fxc_unpol(
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
        let t19 = t17 * t18;
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
        let t36 = 1.0 - 0.3123398257303946694e-2 * t34;
        let t37 = t20 * t20;
        let t38 = t21 * t21;
        let t39 = t38 * t21;
        let t41 = 1.0 / t22 / t39;
        let t42 = t37 * t41;
        let t43 = sigma[ip] * sigma[ip];
        let t44 = t43 * t43;
        let t45 = t44 * sigma[ip];
        let t47 = t29 * t29;
        let t48 = t47 * rho[ip];
        let t49 = t47 * t47;
        let t50 = t49 * t48;
        let t52 = 1.0 / t18 / t50;
        let t56 = 1.0 - 0.14268491327672029207e-10 * t42 * t45 * t26 * t52;
        let t57 = 1.0 / t56;
        let t61 = 1.0 + 0.37270642201834862386e-1 * t34;
        let t64 = 0.1804e1 - 0.56028717948717948718e0 * t36 * t57 - 0.24371282051282051282e0 / t61;
        let t68 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t64);
        let tzk0 = 2.0 * t68;
        zk[ip] += tzk0;
        let t70 = t17 / t30;
        let t74 = t25 * sigma[ip];
        let t75 = t29 * rho[ip];
        let t77 = 1.0 / t30 / t75;
        let t79 = t27 * t77 * t57;
        let t82 = t56 * t56;
        let t83 = 1.0 / t82;
        let t85 = t36 * t83 * t37;
        let t86 = t41 * t45;
        let t87 = t47 * t29;
        let t88 = t49 * t87;
        let t91 = t26 / t18 / t88;
        let t95 = t61 * t61;
        let t97 = 1.0 / t95 * t20;
        let t98 = t97 * t24;
        let t102 = -0.46666666666666666667e-2 * t74 * t79 + 0.10659270348691522892e-9 * t85 * t86 * t91 - 0.24222222222222222223e-1 * t98 * t28 * t77;
        let t107 = piecewise3(t2, 0.0, -t6 * t70 * t64 / 8.0 - 3.0 / 8.0 * t6 * t19 * t102);
        let tvrho0 = 2.0 * rho[ip] * t107 + 2.0 * t68;
        vrho[ip] += tvrho0;
        let t114 = t41 * t44;
        let t115 = t26 * t52;
        let t119 = t24 * t27;
        let t123 = 0.175e-2 * t25 * t27 * t32 * t57 - 0.39972263807593210847e-10 * t85 * t114 * t115 + 0.90833333333333333335e-2 * t97 * t119 * t32;
        let t127 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t123);
        let tvsigma0 = 2.0 * rho[ip] * t127;
        vsigma[ip] += tvsigma0;
        let t132 = t17 / t30 / rho[ip];
        let t140 = 1.0 / t30 / t47;
        let t142 = t27 * t140 * t57;
        let t145 = t44 * t43;
        let t146 = t49 * t49;
        let t148 = 1.0 / t146 / t29;
        let t153 = 1.0 / t82 / t56;
        let t155 = t36 * t153 * t20;
        let t156 = t38 * t38;
        let t159 = 1.0 / t23 / t156 / t38;
        let t160 = t44 * t44;
        let t161 = t160 * t43;
        let t162 = t159 * t161;
        let t163 = t49 * t47;
        let t167 = t27 / t30 / t146 / t163;
        let t171 = t47 * t75;
        let t172 = t49 * t171;
        let t175 = t26 / t18 / t172;
        let t181 = 1.0 / t95 / t61 * t37;
        let t183 = 1.0 / t22 / t21;
        let t184 = t181 * t183;
        let t185 = t43 * t26;
        let t187 = 1.0 / t18 / t171;
        let t194 = 0.17111111111111111111e-1 * t74 * t142 + 0.22456177547295639295e-14 * t145 * t148 * t83 - 0.24334673044738656188e-18 * t155 * t162 * t167 - 0.15278287499791182812e-8 * t85 * t86 * t175 - 0.96296296296296296303e-2 * t184 * t185 * t187 + 0.88814814814814814818e-1 * t98 * t28 * t140;
        let t199 = piecewise3(t2, 0.0, t6 * t132 * t64 / 12.0 - t6 * t70 * t102 / 4.0 - 3.0 / 8.0 * t6 * t19 * t194);
        let tv2rho20 = 2.0 * rho[ip] * t199 + 4.0 * t107;
        v2rho2[ip] += tv2rho20;
        let t207 = t146 * rho[ip];
        let t209 = 1.0 / t207 * t83;
        let t212 = t160 * sigma[ip];
        let t213 = t159 * t212;
        let t214 = t49 * t75;
        let t218 = t27 / t30 / t146 / t214;
        let t226 = 1.0 / t18 / t87;
        let t234 = -0.46666666666666666667e-2 * t25 * t79 - 0.84210665802358647355e-15 * t209 * t45 + 0.91255023917769960709e-19 * t155 * t213 * t218 + 0.53296351743457614463e-9 * t85 * t114 * t91 + 0.36111111111111111113e-2 * t184 * t26 * t226 * sigma[ip] - 0.24222222222222222223e-1 * t97 * t119 * t77;
        let t239 = piecewise3(t2, 0.0, -t6 * t70 * t123 / 8.0 - 3.0 / 8.0 * t6 * t19 * t234);
        let tv2rhosigma0 = 2.0 * rho[ip] * t239 + 2.0 * t127;
        v2rhosigma[ip] += tv2rhosigma0;
        let t243 = 1.0 / t146 * t83;
        let t246 = t159 * t160;
        let t247 = t49 * t29;
        let t251 = t27 / t30 / t146 / t247;
        let t255 = t43 * sigma[ip];
        let t256 = t41 * t255;
        let t260 = t183 * t26;
        let t266 = 0.31578999675884492758e-15 * t243 * t44 - 0.34220633969163735268e-19 * t155 * t246 * t251 - 0.15988905523037284339e-9 * t85 * t256 * t115 - 0.13541666666666666667e-2 * t181 * t260 / t18 / t48;
        let t270 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t266);
        let tv2sigma20 = 2.0 * rho[ip] * t270;
        v2sigma2[ip] += tv2sigma20;
    }
}
