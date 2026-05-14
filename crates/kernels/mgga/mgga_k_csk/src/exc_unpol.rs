//! MGGA_K_CSK exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_csk.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_k_csk_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_csk_a: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = t4 * t4;
        let t6 = M_CBRTPI;
        let t8 = t5 * t6 * M_PI;
        let t9 = 1.0 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t12 = piecewise5(t9, t10, t9, -t10, 0.0);
        let t13 = 1.0 + t12;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * t15;
        let t18 = pow_1_3(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3(t13 <= zeta_threshold, t16 * zeta_threshold, t19 * t13);
        let t22 = pow_1_3(rho[ip]);
        let t23 = t22 * t22;
        let t24 = t21 * t23;
        let t25 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t28 = t27 * t27;
        let t30 = t25 / t28;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t23 / t34;
        let t38 = t30 * t33 * t36;
        let t40 = lapl[ip] * t32;
        let t42 = 1.0 / t23 / rho[ip];
        let t47 = 5.0 / 54.0 * t30 * t40 * t42 - 5.0 / 81.0 * t38;
        let t49 = f64::ln(1.0 - f64::EPSILON);
        let t50 = 1.0 / param_csk_a;
        let t51 = f64::powf(-t49, -t50);
        let t52 = t47 < -t51;
        let t53 = f64::ln(f64::EPSILON);
        let t54 = f64::powf(-t53, -t50);
        let t55 = -t54 < t47;
        let t56 = piecewise3(t55, -t54, t47);
        let t57 = -t51 < t56;
        let t58 = piecewise3(t57, t56, -t51);
        let t59 = f64::abs(t58);
        let t60 = f64::powf(t59, param_csk_a);
        let t61 = 1.0 / t60;
        let t62 = f64::exp(-t61);
        let t63 = 1.0 - t62;
        let t64 = f64::powf(t63, t50);
        let t65 = piecewise5(t52, 0.0, t55, 1.0, t64);
        let t67 = 1.0 + 5.0 / 72.0 * t38 + t47 * t65;
        let t71 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t67);
        let tzk0 = 2.0 * t71;
        zk[ip] += tzk0;
    }
}
