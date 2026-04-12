//! GGA_C_LYP vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_lyp.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_PI};
use crate::math::piecewise::{piecewise3};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_lyp_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3(rho[ip]);
        let t2 = 1.0 / t1;
        let t4 = param_d * t2 + 1.0;
        let t5 = 1.0 / t4;
        let t7 = f64::exp(-param_c * t2);
        let t8 = param_b * t7;
        let t9 = rho[ip] * rho[ip];
        let t10 = t1 * t1;
        let t12 = 1.0 / t10 / t9;
        let t13 = sigma[ip] * t12;
        let t15 = param_d * t5 + param_c;
        let t16 = t15 * t2;
        let t18 = -1.0 / 72.0 - 7.0 / 72.0 * t16;
        let t20 = M_CBRT3;
        let t21 = t20 * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t26 = 1.0 <= zeta_threshold;
        let t27 = zeta_threshold * zeta_threshold;
        let t28 = pow_1_3(zeta_threshold);
        let t29 = t28 * t28;
        let t31 = piecewise3(t26, t29 * t27, 1.0);
        let t35 = 5.0 / 2.0 - t16 / 18.0;
        let t36 = t35 * sigma[ip];
        let t37 = t12 * t31;
        let t40 = t16 - 11.0;
        let t41 = t40 * sigma[ip];
        let t44 = piecewise3(t26, t29 * t27 * zeta_threshold, 1.0);
        let t45 = t12 * t44;
        let t48 = M_CBRT2;
        let t49 = t48 * t48;
        let t50 = sigma[ip] * t49;
        let t53 = piecewise3(t26, t27, 1.0);
        let t54 = t53 * sigma[ip];
        let t56 = t49 * t12 * t31;
        let t62 = -t13 * t18 - 3.0 / 10.0 * t21 * t24 * t31 + t36 * t37 / 8.0 + t41 * t45 / 144.0 - t48 * (4.0 / 3.0 * t50 * t37 - t54 * t56 / 2.0) / 8.0;
        let tzk0 = param_a * (t8 * t5 * t62 - t5);
        zk[ip] += tzk0;
        let t66 = rho[ip] * param_a;
        let t67 = t4 * t4;
        let t68 = 1.0 / t67;
        let t69 = t68 * param_d;
        let t71 = 1.0 / t1 / rho[ip];
        let t74 = param_b * param_c;
        let t75 = t74 * t71;
        let t76 = t7 * t5;
        let t77 = t76 * t62;
        let t80 = t8 * t68;
        let t81 = t62 * param_d;
        let t85 = t9 * rho[ip];
        let t87 = 1.0 / t10 / t85;
        let t88 = sigma[ip] * t87;
        let t91 = param_d * param_d;
        let t92 = t91 * t68;
        let t94 = 1.0 / t10 / rho[ip];
        let t97 = t15 * t71 - t92 * t94;
        let t98 = 7.0 / 216.0 * t97;
        let t100 = t97 / 54.0;
        let t101 = t100 * sigma[ip];
        let t104 = t87 * t31;
        let t108 = -t97 / 3.0;
        let t109 = t108 * sigma[ip];
        let t112 = t87 * t44;
        let t118 = t49 * t87 * t31;
        let t124 = 8.0 / 3.0 * t88 * t18 - t13 * t98 + t101 * t37 / 8.0 - t36 * t104 / 3.0 + t109 * t45 / 144.0 - t41 * t112 / 54.0 - t48 * (-32.0 / 9.0 * t50 * t104 + 4.0 / 3.0 * t54 * t118) / 8.0;
        let t127 = -t69 * t71 / 3.0 + t75 * t77 / 3.0 + t80 * t81 * t71 / 3.0 + t8 * t5 * t124;
        let tvrho0 = t66 * t127 + tzk0;
        vrho[ip] += tvrho0;
        let t129 = t66 * param_b;
        let t138 = t53 * t49;
        let t144 = -t12 * t18 + t35 * t12 * t31 / 8.0 + t40 * t12 * t44 / 144.0 - t48 * (4.0 / 3.0 * t56 - t138 * t37 / 2.0) / 8.0;
        let t145 = t76 * t144;
        let tvsigma0 = t129 * t145;
        vsigma[ip] += tvsigma0;
    }
}
