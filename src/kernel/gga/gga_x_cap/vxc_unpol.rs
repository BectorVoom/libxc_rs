//! GGA_X_CAP vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_cap.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_cap_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_alphaoAx: f64,
    param_c: f64,
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
        let t21 = t20 * t20;
        let t22 = param_alphaoAx * t21;
        let t23 = M_PI * M_PI;
        let t24 = pow_1_3(t23);
        let t25 = 1.0 / t24;
        let t26 = f64::sqrt(sigma[ip]);
        let t28 = t22 * t25 * t26;
        let t29 = M_CBRT2;
        let t31 = 1.0 / t18 / rho[ip];
        let t33 = t21 * t25;
        let t38 = 1.0 + t33 * t26 * t29 * t31 / 12.0;
        let t39 = f64::ln(t38);
        let t41 = param_c * t39 + 1.0;
        let t42 = 1.0 / t41;
        let t43 = t39 * t42;
        let t44 = t29 * t31 * t43;
        let t47 = 1.0 - t28 * t44 / 12.0;
        let t51 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t47);
        let tzk0 = 2.0 * t51;
        zk[ip] += tzk0;
        let t52 = t18 * t18;
        let t54 = t17 / t52;
        let t58 = rho[ip] * rho[ip];
        let t62 = t29 / t18 / t58 * t43;
        let t65 = param_alphaoAx * t20;
        let t66 = t24 * t24;
        let t67 = 1.0 / t66;
        let t68 = t67 * sigma[ip];
        let t69 = t65 * t68;
        let t70 = t29 * t29;
        let t71 = t58 * rho[ip];
        let t73 = 1.0 / t52 / t71;
        let t75 = 1.0 / t38;
        let t76 = t75 * t42;
        let t77 = t70 * t73 * t76;
        let t81 = t65 * t68 * t70;
        let t83 = t41 * t41;
        let t84 = 1.0 / t83;
        let t85 = t84 * param_c;
        let t86 = t85 * t75;
        let t87 = t73 * t39 * t86;
        let t90 = t28 * t62 / 9.0 + t69 * t77 / 18.0 - t81 * t87 / 18.0;
        let t95 = piecewise3(t2, 0.0, -t6 * t54 * t47 / 8.0 - 3.0 / 8.0 * t6 * t19 * t90);
        let tvrho0 = 2.0 * rho[ip] * t95 + 2.0 * t51;
        vrho[ip] += tvrho0;
        let t98 = 1.0 / t26;
        let t100 = t22 * t25 * t98;
        let t103 = t65 * t67;
        let t105 = 1.0 / t52 / t58;
        let t107 = t70 * t105 * t76;
        let t110 = t67 * t70;
        let t111 = t65 * t110;
        let t113 = t105 * t39 * t86;
        let t116 = -t100 * t44 / 24.0 - t103 * t107 / 48.0 + t111 * t113 / 48.0;
        let t120 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t116);
        let tvsigma0 = 2.0 * rho[ip] * t120;
        vsigma[ip] += tvsigma0;
    }
}
