//! GGA_X_CHACHIYO exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_chachiyo.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_chachiyo_exc_pol(
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
        let t19 = t18 + 1.0;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = t5 * t25;
        let t27 = pow_1_3(t6);
        let t28 = t3 * t3;
        let t29 = t2 * t28;
        let t30 = M_CBRT2;
        let t31 = t30 * sigma0;
        let t32 = rho0 * rho0;
        let t33 = pow_1_3(rho0);
        let t34 = t33 * t33;
        let t36 = 1.0 / t34 / t32;
        let t40 = M_PI * M_PI;
        let t41 = t2 * t2;
        let t42 = t41 * t3;
        let t43 = t30 * t30;
        let t44 = f64::sqrt(sigma0);
        let t47 = 1.0 / t33 / rho0;
        let t49 = t42 * t43 * t44 * t47;
        let t51 = t49 / 27.0 + 1.0;
        let t52 = f64::ln(t51);
        let t54 = 2.0 / 81.0 * t29 * t31 * t36 + t40 * t52;
        let t57 = t49 / 9.0 + t40;
        let t58 = 1.0 / t57;
        let t59 = 1.0 / t52;
        let t60 = t58 * t59;
        let t61 = t27 * t54 * t60;
        let t64 = piecewise3(t1, 0.0, -3.0 / 8.0 * t26 * t61);
        let t65 = rho1 <= dens_threshold;
        let t66 = -t16;
        let t68 = piecewise5(t14, t11, t10, t15, t66 * t7);
        let t69 = t68 + 1.0;
        let t70 = t69 <= zeta_threshold;
        let t71 = pow_1_3(t69);
        let t73 = piecewise3(t70, t22, t71 * t69);
        let t74 = t5 * t73;
        let t75 = t30 * sigma2;
        let t76 = rho1 * rho1;
        let t77 = pow_1_3(rho1);
        let t78 = t77 * t77;
        let t80 = 1.0 / t78 / t76;
        let t84 = f64::sqrt(sigma2);
        let t87 = 1.0 / t77 / rho1;
        let t89 = t42 * t43 * t84 * t87;
        let t91 = t89 / 27.0 + 1.0;
        let t92 = f64::ln(t91);
        let t94 = 2.0 / 81.0 * t29 * t75 * t80 + t40 * t92;
        let t97 = t89 / 9.0 + t40;
        let t98 = 1.0 / t97;
        let t99 = 1.0 / t92;
        let t100 = t98 * t99;
        let t101 = t27 * t94 * t100;
        let t104 = piecewise3(t65, 0.0, -3.0 / 8.0 * t74 * t101);
        let tzk0 = t64 + t104;
        zk[ip] += tzk0;
    }
}
