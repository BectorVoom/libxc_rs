//! MGGA_X_MVSB exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mvsb.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_mvsb_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_b: f64,
    param_c1: f64,
    param_e1: f64,
    param_k0: f64,
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
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = t6 * t26;
        let t28 = pow_1_3(t7);
        let t29 = pow_1_3(rho0);
        let t30 = t29 * t29;
        let t32 = 1.0 / t30 / rho0;
        let t33 = tau0 * t32;
        let t34 = rho0 * rho0;
        let t36 = 1.0 / t30 / t34;
        let t39 = t33 - sigma0 * t36 / 8.0;
        let t40 = M_CBRT6;
        let t41 = t40 * t40;
        let t42 = M_PI * M_PI;
        let t43 = pow_1_3(t42);
        let t44 = t43 * t43;
        let t46 = 3.0 / 10.0 * t41 * t44;
        let t47 = t33 - t46;
        let t48 = 1.0 / t47;
        let t51 = param_k0 * (-t39 * t48 + 1.0);
        let t52 = t39 * t39;
        let t53 = param_e1 * t52;
        let t54 = t47 * t47;
        let t55 = 1.0 / t54;
        let t57 = t53 * t55 + 1.0;
        let t58 = t57 * t57;
        let t59 = t52 * t52;
        let t60 = param_c1 * t59;
        let t61 = t54 * t54;
        let t62 = 1.0 / t61;
        let t64 = t60 * t62 + t58;
        let t65 = pow_1_4(t64);
        let t66 = 1.0 / t65;
        let t68 = t51 * t66 + 1.0;
        let t70 = param_b * t41;
        let t72 = 1.0 / t43 / t42;
        let t73 = sigma0 * sigma0;
        let t74 = t72 * t73;
        let t75 = t34 * t34;
        let t76 = t75 * rho0;
        let t78 = 1.0 / t29 / t76;
        let t82 = 1.0 + t70 * t74 * t78 / 576.0;
        let t83 = f64::powf(t82, 1.0 / 8.0);
        let t84 = 1.0 / t83;
        let t85 = t28 * t68 * t84;
        let t88 = piecewise3(t2, 0.0, -3.0 / 8.0 * t27 * t85);
        let t89 = rho1 <= dens_threshold;
        let t90 = -t17;
        let t92 = piecewise5(t15, t12, t11, t16, t90 * t8);
        let t93 = 1.0 + t92;
        let t94 = t93 <= zeta_threshold;
        let t95 = pow_1_3(t93);
        let t97 = piecewise3(t94, t23, t95 * t93);
        let t98 = t6 * t97;
        let t99 = pow_1_3(rho1);
        let t100 = t99 * t99;
        let t102 = 1.0 / t100 / rho1;
        let t103 = tau1 * t102;
        let t104 = rho1 * rho1;
        let t106 = 1.0 / t100 / t104;
        let t109 = t103 - sigma2 * t106 / 8.0;
        let t110 = t103 - t46;
        let t111 = 1.0 / t110;
        let t114 = param_k0 * (-t109 * t111 + 1.0);
        let t115 = t109 * t109;
        let t116 = param_e1 * t115;
        let t117 = t110 * t110;
        let t118 = 1.0 / t117;
        let t120 = t116 * t118 + 1.0;
        let t121 = t120 * t120;
        let t122 = t115 * t115;
        let t123 = param_c1 * t122;
        let t124 = t117 * t117;
        let t125 = 1.0 / t124;
        let t127 = t123 * t125 + t121;
        let t128 = pow_1_4(t127);
        let t129 = 1.0 / t128;
        let t131 = t114 * t129 + 1.0;
        let t133 = sigma2 * sigma2;
        let t134 = t72 * t133;
        let t135 = t104 * t104;
        let t136 = t135 * rho1;
        let t138 = 1.0 / t99 / t136;
        let t142 = 1.0 + t70 * t134 * t138 / 576.0;
        let t143 = f64::powf(t142, 1.0 / 8.0);
        let t144 = 1.0 / t143;
        let t145 = t28 * t131 * t144;
        let t148 = piecewise3(t89, 0.0, -3.0 / 8.0 * t98 * t145);
        let tzk0 = t88 + t148;
        zk[ip] += tzk0;
    }
}
