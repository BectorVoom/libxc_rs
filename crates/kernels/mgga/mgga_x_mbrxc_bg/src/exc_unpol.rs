//! MGGA_X_MBRXC_BG exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mbrxc_bg.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::mbrxc::{xc_mgga_x_mbrxc_get_x};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_mbrxc_bg_exc_unpol(
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
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = M_CBRT2;
        let t22 = t21 * t21;
        let t23 = tau[ip] * t22;
        let t24 = t19 * t19;
        let t26 = 1.0 / t24 / rho[ip];
        let t29 = M_CBRT6;
        let t30 = t29 * t29;
        let t31 = M_PI * M_PI;
        let t32 = pow_1_3(t31);
        let t33 = t32 * t32;
        let t36 = sigma[ip] * t22;
        let t37 = rho[ip] * rho[ip];
        let t39 = 1.0 / t24 / t37;
        let t42 = sigma[ip] * sigma[ip];
        let t43 = t42 * t21;
        let t44 = t37 * t37;
        let t45 = t44 * rho[ip];
        let t47 = 1.0 / t19 / t45;
        let t50 = 0.149492e0 * t23 * t26 - 3.0 / 10.0 * t30 * t33 + 0.147e0 * t36 * t39 + 0.64e-2 * t43 * t47;
        let t51 = xc_mgga_x_mbrxc_get_x(t50);
        let t52 = pow_1_4(f64::EPSILON);
        let t53 = t51 < t52;
        let t54 = pow_1_3(32.0);
        let t55 = t54 * t5;
        let t56 = t4 * t4;
        let t58 = pow_1_3(1.0 / M_PI);
        let t59 = 1.0 / t58;
        let t60 = t56 * t59;
        let t61 = M_CBRT4;
        let t62 = t60 * t61;
        let t63 = t55 * t62;
        let t65 = t55 * t56;
        let t66 = t59 * t61;
        let t67 = t51 * t51;
        let t68 = t66 * t67;
        let t71 = t67 * t51;
        let t72 = t66 * t71;
        let t75 = t67 * t67;
        let t76 = t66 * t75;
        let t79 = t75 * t51;
        let t80 = t66 * t79;
        let t83 = t75 * t67;
        let t84 = t66 * t83;
        let t92 = t55 * t60;
        let t93 = t52 < t51;
        let t94 = piecewise3(t93, t51, t52);
        let t96 = f64::exp(t94 / 3.0);
        let t97 = t61 * t96;
        let t98 = f64::exp(-t94);
        let t99 = t94 * t94;
        let t101 = t99 + 5.0 * t94 + 8.0;
        let t102 = t98 * t101;
        let t103 = 8.0 - t102;
        let t104 = 1.0 / t94;
        let t105 = t103 * t104;
        let t106 = 1.0 + t94;
        let t107 = pow_1_3(t106);
        let t108 = 1.0 / t107;
        let t109 = t105 * t108;
        let t113 = piecewise3(t53, -t63 / 12.0 - t65 * t68 / 108.0 + t65 * t72 / 108.0 - 13.0 / 1620.0 * t65 * t76 + 67.0 / 9720.0 * t65 * t80 - 52.0 / 8505.0 * t65 * t84 + 1811.0 / 326592.0 * t65 * t66 * t75 * t71, -t92 * t97 * t109 / 36.0);
        let t117 = piecewise3(t3, 0.0, 3.0 / 16.0 * t7 * t20 * t113);
        let tzk0 = 2.0 * t117;
        zk[ip] += tzk0;
    }
}
