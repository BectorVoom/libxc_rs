//! MGGA_X_TB09 vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_vxc/mgga_x_tb09.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::br89::{xc_mgga_x_br89_get_x};
use libxc_kernel_math::constants::{M_CBRT2, M_CBRTPI, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_tb09_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_alpha: f64,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = M_CBRTPI;
        let t3 = param_c * t2;
        let t4 = pow_1_3::<f64>(rho0);
        let t5 = t4 * t4;
        let t7 = 1.0 / t5 / rho0;
        let t10 = tau0 * t7;
        let t12 = rho0 * rho0;
        let t14 = 1.0 / t5 / t12;
        let t17 = lapl0 * t7 / 6.0 - 0.53333333333333333333e0 * t10 + 0.66666666666666666667e-1 * sigma0 * t14;
        let t18 = f64::abs(t17);
        let t19 = t18 < 0.5e-12;
        let t20 = 0.0 < t17;
        let t21 = piecewise3::<f64>(t20, 0.5e-12, -0.5e-12);
        let t22 = piecewise3::<f64>(t19, t21, t17);
        let t23 = xc_mgga_x_br89_get_x::<f64>(t22);
        let t25 = f64::exp(t23 / 3.0);
        let t26 = f64::exp(-t23);
        let t28 = 1.0 + t23 / 2.0;
        let t29 = t26 * t28;
        let t30 = 1.0 - t29;
        let t31 = t25 * t30;
        let t32 = 1.0 / t23;
        let t33 = t31 * t32;
        let t38 = f64::sqrt(15.0);
        let t39 = (3.0 * param_c - 2.0) * t38;
        let t40 = 1.0 / M_PI;
        let t41 = M_SQRT2;
        let t42 = t40 * t41;
        let t43 = param_alpha * sigma0;
        let t46 = t10 - t43 * t14 / 8.0;
        let t47 = 0.1e-9 < t46;
        let t48 = piecewise3::<f64>(t47, t46, 0.1e-9);
        let t49 = f64::sqrt(t48);
        let t53 = -2.0 * t3 * t33 + t39 * t42 * t49 / 6.0;
        let tvrho0 = t53 * t4;
        vrho[ip * 2] += tvrho0;
        let t54 = pow_1_3::<f64>(rho1);
        let t55 = t54 * t54;
        let t57 = 1.0 / t55 / rho1;
        let t60 = tau1 * t57;
        let t62 = rho1 * rho1;
        let t64 = 1.0 / t55 / t62;
        let t67 = lapl1 * t57 / 6.0 - 0.53333333333333333333e0 * t60 + 0.66666666666666666667e-1 * sigma2 * t64;
        let t68 = f64::abs(t67);
        let t69 = t68 < 0.5e-12;
        let t70 = 0.0 < t67;
        let t71 = piecewise3::<f64>(t70, 0.5e-12, -0.5e-12);
        let t72 = piecewise3::<f64>(t69, t71, t67);
        let t73 = xc_mgga_x_br89_get_x::<f64>(t72);
        let t75 = f64::exp(t73 / 3.0);
        let t76 = f64::exp(-t73);
        let t78 = 1.0 + t73 / 2.0;
        let t79 = t76 * t78;
        let t80 = 1.0 - t79;
        let t81 = t75 * t80;
        let t82 = 1.0 / t73;
        let t83 = t81 * t82;
        let t86 = param_alpha * sigma2;
        let t89 = t60 - t86 * t64 / 8.0;
        let t90 = 0.1e-9 < t89;
        let t91 = piecewise3::<f64>(t90, t89, 0.1e-9);
        let t92 = f64::sqrt(t91);
        let t96 = -2.0 * t3 * t83 + t39 * t42 * t92 / 6.0;
        let tvrho1 = t96 * t54;
        vrho[ip * 2 + 1] += tvrho1;
    }
}
