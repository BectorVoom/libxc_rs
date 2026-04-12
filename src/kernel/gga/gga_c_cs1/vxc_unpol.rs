//! GGA_C_CS1 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_cs1.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2};
use crate::math::piecewise::{piecewise3};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_cs1_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3(rho[ip]);
        let t2 = 1.0 / t1;
        let t4 = 1.0 + 0.349e0 * t2;
        let t5 = 1.0 / t4;
        let t6 = sigma[ip] * sigma[ip];
        let t7 = rho[ip] * rho[ip];
        let t8 = t7 * t7;
        let t9 = t8 * rho[ip];
        let t11 = 1.0 / t1 / t9;
        let t13 = t1 * t1;
        let t15 = 1.0 / t13 / t7;
        let t18 = 1.0 + 0.6e-2 * sigma[ip] * t15;
        let t19 = t18 * t18;
        let t20 = 1.0 / t19;
        let t23 = -0.159068e0 + 0.286308e-6 * t6 * t11 * t20;
        let t25 = t5 * t23 / 4.0;
        let t27 = piecewise3(1.0 <= zeta_threshold, zeta_threshold, 1.0);
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t30 = t27 * t29;
        let t33 = t29 * t1 / 2.0 + 0.349e0;
        let t34 = 1.0 / t33;
        let t35 = t1 * t34;
        let t36 = t6 * t28;
        let t37 = sigma[ip] * t29;
        let t40 = 1.0 + 0.6e-2 * t37 * t15;
        let t41 = t40 * t40;
        let t42 = 1.0 / t41;
        let t43 = t11 * t42;
        let t46 = -0.18897e-1 + 0.1117728e-4 * t36 * t43;
        let t49 = t30 * t35 * t46 / 2.0;
        let tzk0 = t25 + t49;
        zk[ip] += tzk0;
        let t50 = t4 * t4;
        let t51 = 1.0 / t50;
        let t52 = t51 * t23;
        let t54 = 1.0 / t1 / rho[ip];
        let t55 = t52 * t54;
        let t57 = t8 * t7;
        let t59 = 1.0 / t1 / t57;
        let t63 = t6 * sigma[ip];
        let t64 = t8 * t8;
        let t65 = t64 * rho[ip];
        let t66 = 1.0 / t65;
        let t67 = t63 * t66;
        let t69 = 1.0 / t19 / t18;
        let t72 = -0.1526976e-5 * t6 * t59 * t20 + 0.9161856e-8 * t67 * t69;
        let t73 = t5 * t72;
        let t76 = 1.0 / t13 * t34;
        let t78 = t30 * t76 * t46;
        let t80 = t27 * t28;
        let t81 = t33 * t33;
        let t82 = 1.0 / t81;
        let t83 = t2 * t82;
        let t85 = t80 * t83 * t46;
        let t87 = t59 * t42;
        let t91 = 1.0 / t41 / t40;
        let t94 = -0.5961216e-4 * t36 * t87 + 0.71534592e-6 * t67 * t91;
        let t96 = t30 * t35 * t94;
        let tvrho0 = t25 + t49 + rho[ip] * (0.29083333333333333332e-1 * t55 + t73 / 4.0 + t78 / 6.0 - t85 / 6.0 + t96 / 2.0);
        vrho[ip] += tvrho0;
        let t103 = 1.0 / t64;
        let t104 = t6 * t103;
        let t107 = 0.572616e-6 * sigma[ip] * t11 * t20 - 0.3435696e-8 * t104 * t69;
        let t109 = t5 * t107 / 4.0;
        let t110 = sigma[ip] * t28;
        let t115 = 0.2235456e-4 * t110 * t43 - 0.26825472e-6 * t104 * t91;
        let t118 = t30 * t35 * t115 / 2.0;
        let tvsigma0 = rho[ip] * (t109 + t118);
        vsigma[ip] += tvsigma0;
    }
}
