//! MGGA_X_TH vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_th.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_th_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRTPI;
        let t5 = t4 * t4;
        let t6 = 1.0 <= zeta_threshold;
        let t7 = zeta_threshold - 1.0;
        let t9 = piecewise5::<f64>(t6, t7, t6, -t7, 0.0);
        let t10 = 1.0 + t9;
        let t12 = pow_1_3::<f64>(zeta_threshold);
        let t14 = pow_1_3::<f64>(t10);
        let t16 = piecewise3::<f64>(t10 <= zeta_threshold, t12 * zeta_threshold, t14 * t10);
        let t17 = t5 * t16;
        let t18 = rho[ip] * rho[ip];
        let t19 = 1.0 / tau[ip];
        let t22 = M_CBRT2;
        let t23 = 1.0 / rho[ip];
        let t30 = pow_1_3::<f64>(1.0 / M_PI);
        let t31 = 1.0 / t30;
        let t32 = M_CBRT4;
        let t33 = t31 * t32;
        let t34 = t22 * (1.0 + 7.0 / 216.0 * sigma[ip] * t23 * t19) * t33;
        let t37 = piecewise3::<f64>(t3, 0.0, -27.0 / 160.0 * t17 * t18 * t19 * t34);
        let tzk0 = 2.0 * t37;
        zk[ip] += tzk0;
        let t42 = tau[ip] * tau[ip];
        let t43 = 1.0 / t42;
        let t44 = t17 * t43;
        let t46 = t22 * sigma[ip] * t33;
        let t50 = piecewise3::<f64>(t3, 0.0, -27.0 / 80.0 * t17 * rho[ip] * t19 * t34 + 7.0 / 1280.0 * t44 * t46);
        let tvrho0 = 2.0 * rho[ip] * t50 + 2.0 * t37;
        vrho[ip] += tvrho0;
        let t53 = t17 * rho[ip];
        let t58 = piecewise3::<f64>(t3, 0.0, -7.0 / 1280.0 * t53 * t43 * t22 * t33);
        let tvsigma0 = 2.0 * rho[ip] * t58;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t65 = 1.0 / t42 / tau[ip];
        let t67 = t17 * rho[ip] * t65;
        let t71 = piecewise3::<f64>(t3, 0.0, 27.0 / 160.0 * t17 * t18 * t43 * t34 + 7.0 / 1280.0 * t67 * t46);
        let tvtau0 = 2.0 * rho[ip] * t71;
        vtau[ip] += tvtau0;
    }
}
