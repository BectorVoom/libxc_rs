//! GGA_X_RGE2 exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_rge2.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_rge2_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t28 = M_CBRT6;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = t30 * t30;
        let t33 = t28 / t31;
        let t34 = rho0 * rho0;
        let t35 = pow_1_3(rho0);
        let t36 = t35 * t35;
        let t38 = 1.0 / t36 / t34;
        let t42 = t28 * t28;
        let t44 = 1.0 / t30 / t29;
        let t45 = t42 * t44;
        let t46 = sigma0 * sigma0;
        let t47 = t34 * t34;
        let t48 = t47 * rho0;
        let t50 = 1.0 / t35 / t48;
        let t54 = 0.804e0 + 5.0 / 972.0 * t33 * sigma0 * t38 + 0.32911784453572541027e-4 * t45 * t46 * t50;
        let t57 = 0.1804e1 - 0.646416e0 / t54;
        let t61 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t25 * t26 * t57);
        let t62 = rho1 <= dens_threshold;
        let t63 = -t16;
        let t65 = piecewise5(t14, t11, t10, t15, t63 * t7);
        let t66 = 1.0 + t65;
        let t67 = t66 <= zeta_threshold;
        let t68 = pow_1_3(t66);
        let t70 = piecewise3(t67, t22, t68 * t66);
        let t72 = rho1 * rho1;
        let t73 = pow_1_3(rho1);
        let t74 = t73 * t73;
        let t76 = 1.0 / t74 / t72;
        let t80 = sigma2 * sigma2;
        let t81 = t72 * t72;
        let t82 = t81 * rho1;
        let t84 = 1.0 / t73 / t82;
        let t88 = 0.804e0 + 5.0 / 972.0 * t33 * sigma2 * t76 + 0.32911784453572541027e-4 * t45 * t80 * t84;
        let t91 = 0.1804e1 - 0.646416e0 / t88;
        let t95 = piecewise3(t62, 0.0, -3.0 / 8.0 * t5 * t70 * t26 * t91);
        let tzk0 = t61 + t95;
        zk[ip] += tzk0;
    }
}
