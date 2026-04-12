//! GGA_X_2D_B88 exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_2d_b88.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_PI, M_SQRT2};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_2d_b88_exc_pol(
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
        let t2 = f64::sqrt(M_PI);
        let t3 = 1.0 / t2;
        let t4 = rho0 + rho1;
        let t5 = 1.0 / t4;
        let t8 = 2.0 * rho0 * t5 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t12 = 2.0 * rho1 * t5 <= zeta_threshold;
        let t13 = -t9;
        let t14 = rho0 - rho1;
        let t16 = piecewise5(t8, t9, t12, t13, t14 * t5);
        let t17 = 1.0 + t16;
        let t18 = t17 <= zeta_threshold;
        let t19 = f64::sqrt(zeta_threshold);
        let t20 = t19 * zeta_threshold;
        let t21 = f64::sqrt(t17);
        let t22 = t21 * t17;
        let t23 = piecewise3(t18, t20, t22);
        let t24 = t3 * t23;
        let t25 = M_SQRT2;
        let t26 = f64::sqrt(t4);
        let t27 = t25 * t26;
        let t28 = rho0 * rho0;
        let t29 = t28 * rho0;
        let t30 = 1.0 / t29;
        let t31 = sigma0 * t30;
        let t32 = f64::sqrt(sigma0);
        let t33 = f64::sqrt(rho0);
        let t35 = 1.0 / t33 / rho0;
        let t36 = t32 * t35;
        let t37 = f64::ln(t36 + f64::sqrt(t36 * t36 + 1.0));
        let t40 = 1.0 + 0.56e-1 * t36 * t37;
        let t41 = 1.0 / t40;
        let t44 = 1.0 + 0.46526913586269795717e-2 * t31 * t41;
        let t45 = t27 * t44;
        let t48 = piecewise3(t1, 0.0, -2.0 / 3.0 * t24 * t45);
        let t49 = rho1 <= dens_threshold;
        let t50 = -t14;
        let t52 = piecewise5(t12, t9, t8, t13, t50 * t5);
        let t53 = 1.0 + t52;
        let t54 = t53 <= zeta_threshold;
        let t55 = f64::sqrt(t53);
        let t56 = t55 * t53;
        let t57 = piecewise3(t54, t20, t56);
        let t58 = t3 * t57;
        let t59 = rho1 * rho1;
        let t60 = t59 * rho1;
        let t61 = 1.0 / t60;
        let t62 = sigma2 * t61;
        let t63 = f64::sqrt(sigma2);
        let t64 = f64::sqrt(rho1);
        let t66 = 1.0 / t64 / rho1;
        let t67 = t63 * t66;
        let t68 = f64::ln(t67 + f64::sqrt(t67 * t67 + 1.0));
        let t71 = 1.0 + 0.56e-1 * t67 * t68;
        let t72 = 1.0 / t71;
        let t75 = 1.0 + 0.46526913586269795717e-2 * t62 * t72;
        let t76 = t27 * t75;
        let t79 = piecewise3(t49, 0.0, -2.0 / 3.0 * t58 * t76);
        let tzk0 = t48 + t79;
        zk[ip] += tzk0;
    }
}
