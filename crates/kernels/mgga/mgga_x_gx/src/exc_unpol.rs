//! MGGA_X_GX exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gx.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{Heaviside, piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_gx_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_alphainf: f64,
    param_c0: f64,
    param_c1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5::<f64>(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3::<f64>(zeta_threshold);
        let t16 = pow_1_3::<f64>(t12);
        let t18 = piecewise3::<f64>(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3::<f64>(rho[ip]);
        let t20 = t18 * t19;
        let t21 = M_CBRT2;
        let t22 = t4 * t4;
        let t24 = M_CBRT4;
        let t26 = 8.0 / 27.0 * t21 * t22 * t24;
        let t27 = t21 * t21;
        let t28 = tau[ip] * t27;
        let t29 = t19 * t19;
        let t31 = 1.0 / t29 / rho[ip];
        let t33 = sigma[ip] * t27;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t29 / t34;
        let t39 = t28 * t31 - t33 * t36 / 8.0;
        let t40 = M_CBRT6;
        let t42 = M_PI * M_PI;
        let t43 = pow_1_3::<f64>(t42);
        let t44 = t43 * t43;
        let t45 = 1.0 / t44;
        let t46 = t39 * t40 * t45;
        let t48 = t40 * t45;
        let t51 = param_c0 + 5.0 / 9.0 * param_c1 * t39 * t48;
        let t52 = param_c0 + param_c1 - 1.0;
        let t56 = 1.0 + 5.0 / 9.0 * t52 * t39 * t48;
        let t57 = 1.0 / t56;
        let t59 = 1.0 - t26;
        let t60 = t51 * t57 * t59;
        let t63 = t26 + 5.0 / 9.0 * t46 * t60;
        let t64 = 5.0 / 9.0 * t46;
        let t65 = 1.0 - t64;
        let t66 = Heaviside(t65);
        let t68 = 1.0 - param_alphainf;
        let t69 = t68 * t65;
        let t70 = 1.0 + t64;
        let t71 = 1.0 / t70;
        let t73 = t69 * t71 + 1.0;
        let t74 = -t65;
        let t75 = Heaviside(t74);
        let t77 = t63 * t66 + t73 * t75;
        let t81 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t77);
        let tzk0 = 2.0 * t81;
        zk[ip] += tzk0;
    }
}
