//! MGGA_X_VT84 exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_vt84.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_vt84_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
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
        let t21 = sigma[ip] * sigma[ip];
        let t22 = t21 * sigma[ip];
        let t23 = rho[ip] * rho[ip];
        let t24 = t23 * rho[ip];
        let t25 = 1.0 / t24;
        let t26 = t22 * t25;
        let t27 = tau[ip] * tau[ip];
        let t28 = t27 * tau[ip];
        let t29 = 1.0 / t28;
        let t30 = 1.0 / t23;
        let t31 = t21 * t30;
        let t32 = 1.0 / t27;
        let t33 = t31 * t32;
        let t35 = 1.0 + t33 / 64.0;
        let t36 = t35 * t35;
        let t37 = 1.0 / t36;
        let t38 = t29 * t37;
        let t42 = M_CBRT6;
        let t43 = (10.0 / 81.0 + 0.419826171875e-2 * t26 * t38) * t42;
        let t44 = M_PI * M_PI;
        let t45 = pow_1_3::<f64>(t44);
        let t46 = t45 * t45;
        let t47 = 1.0 / t46;
        let t48 = t43 * t47;
        let t49 = M_CBRT2;
        let t50 = t49 * t49;
        let t51 = sigma[ip] * t50;
        let t52 = t19 * t19;
        let t54 = 1.0 / t52 / t23;
        let t55 = t51 * t54;
        let t58 = tau[ip] * t50;
        let t60 = 1.0 / t52 / rho[ip];
        let t63 = t58 * t60 - t55 / 8.0;
        let t64 = t63 * t42;
        let t67 = 5.0 / 9.0 * t64 * t47 - 1.0;
        let t68 = t47 * t67;
        let t71 = 1.0 + 0.22222222222222222222e0 * t64 * t68;
        let t72 = f64::sqrt(t71);
        let t73 = 1.0 / t72;
        let t76 = t42 * t47;
        let t77 = t76 * t55;
        let t79 = 9.0 / 20.0 * t67 * t73 + t77 / 36.0;
        let t80 = t79 * t79;
        let t83 = t42 * t42;
        let t85 = 1.0 / t45 / t44;
        let t86 = t83 * t85;
        let t87 = t21 * t49;
        let t88 = t23 * t23;
        let t89 = t88 * rho[ip];
        let t91 = 1.0 / t19 / t89;
        let t93 = t86 * t87 * t91;
        let t95 = 162.0 * t33 + 100.0 * t93;
        let t96 = f64::sqrt(t95);
        let t101 = t88 * t88;
        let t102 = 1.0 / t101;
        let t105 = t48 * t55 / 24.0 + 146.0 / 2025.0 * t80 - 73.0 / 97200.0 * t79 * t96 + 0.5301186990888922759e-4 * t93 + 0.19577914932045745128e-2 * t33 + 0.43721079261097766676e-5 * t22 * t102;
        let t107 = 1.0 + 0.58733744796137235383e-1 * t77;
        let t108 = t107 * t107;
        let t109 = 1.0 / t108;
        let t110 = t105 * t109;
        let t112 = f64::exp(-0.1863e-3 * t110);
        let t113 = 1.0 + t110;
        let t114 = 1.0 / t113;
        let t115 = t112 * t114;
        let t117 = t105 * t105;
        let t118 = t108 * t108;
        let t119 = 1.0 / t118;
        let t122 = f64::exp(-0.150903e-2 * t117 * t119);
        let t123 = 1.0 - t122;
        let t124 = 1.0 / t105;
        let t127 = 10.0 / 81.0 * t124 * t108 - 1.0;
        let t129 = t110 * t115 + t123 * t127 + 1.0;
        let t133 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t129);
        let tzk0 = 2.0 * t133;
        zk[ip] += tzk0;
    }
}
