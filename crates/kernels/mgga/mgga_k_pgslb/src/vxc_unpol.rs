//! MGGA_K_PGSLB vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_pgslb.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_k_pgslb_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_pgslb_beta: f64,
    param_pgslb_mu: f64,
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
        let t12 = piecewise5::<f64>(t9, t10, t9, -t10, 0.0);
        let t13 = 1.0 + t12;
        let t15 = pow_1_3::<f64>(zeta_threshold);
        let t16 = t15 * t15;
        let t18 = pow_1_3::<f64>(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3::<f64>(t13 <= zeta_threshold, t16 * zeta_threshold, t19 * t13);
        let t22 = pow_1_3::<f64>(rho[ip]);
        let t23 = t22 * t22;
        let t24 = t21 * t23;
        let t25 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3::<f64>(t26);
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t30 = t25 * t29;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t23 / t34;
        let t37 = t33 * t36;
        let t41 = param_pgslb_mu * t25 * t29;
        let t44 = f64::exp(-t41 * t37 / 24.0);
        let t45 = t25 * t25;
        let t46 = param_pgslb_beta * t45;
        let t48 = 1.0 / t27 / t26;
        let t49 = t46 * t48;
        let t50 = lapl[ip] * lapl[ip];
        let t51 = t50 * t31;
        let t52 = t34 * rho[ip];
        let t54 = 1.0 / t22 / t52;
        let t58 = 5.0 / 72.0 * t30 * t37 + t44 + t49 * t51 * t54 / 288.0;
        let t62 = piecewise3::<f64>(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t58);
        let tzk0 = 2.0 * t62;
        zk[ip] += tzk0;
        let t64 = t21 / t22;
        let t69 = 1.0 / t23 / t52;
        let t77 = t34 * t34;
        let t83 = -5.0 / 27.0 * t30 * t33 * t69 + t41 * t33 * t69 * t44 / 9.0 - 5.0 / 432.0 * t49 * t51 / t22 / t77;
        let t88 = piecewise3::<f64>(t3, 0.0, t8 * t64 * t58 / 10.0 + 3.0 / 20.0 * t8 * t24 * t83);
        let tvrho0 = 2.0 * rho[ip] * t88 + 2.0 * t62;
        vrho[ip] += tvrho0;
        let t91 = t32 * t36;
        let t97 = 5.0 / 72.0 * t30 * t91 - t41 * t91 * t44 / 24.0;
        let t101 = piecewise3::<f64>(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t97);
        let tvsigma0 = 2.0 * rho[ip] * t101;
        vsigma[ip] += tvsigma0;
        let t104 = t8 * t21 * t36;
        let t107 = t46 * t48 * lapl[ip] * t31;
        let t110 = piecewise3::<f64>(t3, 0.0, t104 * t107 / 960.0);
        let tvlapl0 = 2.0 * rho[ip] * t110;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
    }
}
