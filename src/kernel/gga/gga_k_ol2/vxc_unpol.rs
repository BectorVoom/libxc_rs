//! GGA_K_OL2 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_ol2.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_ol2_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_aa: f64,
    param_bb: f64,
    param_cc: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = param_bb * sigma[ip];
        let t25 = M_CBRT2;
        let t26 = t25 * t25;
        let t27 = rho[ip] * rho[ip];
        let t29 = 1.0 / t22 / t27;
        let t30 = t26 * t29;
        let t33 = f64::sqrt(sigma[ip]);
        let t34 = param_cc * t33;
        let t36 = 1.0 / t21 / rho[ip];
        let t41 = 4.0 * t33 * t25 * t36 + t25;
        let t42 = 1.0 / t41;
        let t43 = t25 * t36 * t42;
        let t45 = param_aa + 0.13888888888888888889e-1 * t24 * t30 + t34 * t43;
        let t49 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t45);
        let tzk0 = 2.0 * t49;
        zk[ip] += tzk0;
        let t51 = t20 / t21;
        let t55 = t27 * rho[ip];
        let t57 = 1.0 / t22 / t55;
        let t58 = t26 * t57;
        let t62 = 1.0 / t21 / t27;
        let t64 = t25 * t62 * t42;
        let t67 = param_cc * sigma[ip];
        let t68 = t41 * t41;
        let t69 = 1.0 / t68;
        let t70 = t58 * t69;
        let t73 = -0.37037037037037037037e-1 * t24 * t58 - 4.0 / 3.0 * t34 * t64 + 16.0 / 3.0 * t67 * t70;
        let t78 = piecewise3(t2, 0.0, t7 * t51 * t45 / 10.0 + 3.0 / 20.0 * t7 * t23 * t73);
        let tvrho0 = 2.0 * rho[ip] * t78 + 2.0 * t49;
        vrho[ip] += tvrho0;
        let t81 = param_bb * t26;
        let t84 = 1.0 / t33;
        let t85 = param_cc * t84;
        let t88 = param_cc * t26;
        let t92 = 0.13888888888888888889e-1 * t81 * t29 + t85 * t43 / 2.0 - 2.0 * t88 * t29 * t69;
        let t96 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t92);
        let tvsigma0 = 2.0 * rho[ip] * t96;
        vsigma[ip] += tvsigma0;
    }
}
