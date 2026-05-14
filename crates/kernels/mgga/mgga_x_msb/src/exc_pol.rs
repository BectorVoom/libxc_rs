//! MGGA_X_MSB exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_msb.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_msb_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_b: f64,
    param_c: f64,
    param_kappa: f64,
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
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t29 = M_CBRT6;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t40 = sigma0 * t39;
        let t42 = 5.0 / 972.0 * t34 * t40;
        let t43 = param_kappa + t42;
        let t47 = param_kappa * (1.0 - param_kappa / t43);
        let t49 = 1.0 / t37 / rho0;
        let t50 = tau0 * t49;
        let t52 = t50 - t40 / 8.0;
        let t53 = t52 * t52;
        let t54 = t29 * t29;
        let t56 = 3.0 / 10.0 * t54 * t32;
        let t57 = t50 + t56;
        let t58 = t57 * t57;
        let t59 = 1.0 / t58;
        let t62 = -4.0 * t53 * t59 + 1.0;
        let t63 = t62 * t62;
        let t64 = t63 * t62;
        let t65 = t53 * t52;
        let t66 = t58 * t57;
        let t67 = 1.0 / t66;
        let t70 = t53 * t53;
        let t72 = param_b * t70 * t53;
        let t73 = t58 * t58;
        let t75 = 1.0 / t73 / t58;
        let t78 = 8.0 * t65 * t67 + 64.0 * t72 * t75 + 1.0;
        let t79 = 1.0 / t78;
        let t80 = t64 * t79;
        let t81 = param_kappa + t42 + param_c;
        let t86 = param_kappa * (1.0 - param_kappa / t81) - t47;
        let t88 = t80 * t86 + t47 + 1.0;
        let t92 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t88);
        let t93 = rho1 <= dens_threshold;
        let t94 = -t17;
        let t96 = piecewise5(t15, t12, t11, t16, t94 * t8);
        let t97 = 1.0 + t96;
        let t98 = t97 <= zeta_threshold;
        let t99 = pow_1_3(t97);
        let t101 = piecewise3(t98, t23, t99 * t97);
        let t102 = t101 * t27;
        let t103 = rho1 * rho1;
        let t104 = pow_1_3(rho1);
        let t105 = t104 * t104;
        let t107 = 1.0 / t105 / t103;
        let t108 = sigma2 * t107;
        let t110 = 5.0 / 972.0 * t34 * t108;
        let t111 = param_kappa + t110;
        let t115 = param_kappa * (1.0 - param_kappa / t111);
        let t117 = 1.0 / t105 / rho1;
        let t118 = tau1 * t117;
        let t120 = t118 - t108 / 8.0;
        let t121 = t120 * t120;
        let t122 = t118 + t56;
        let t123 = t122 * t122;
        let t124 = 1.0 / t123;
        let t127 = -4.0 * t121 * t124 + 1.0;
        let t128 = t127 * t127;
        let t129 = t128 * t127;
        let t130 = t121 * t120;
        let t131 = t123 * t122;
        let t132 = 1.0 / t131;
        let t135 = t121 * t121;
        let t137 = param_b * t135 * t121;
        let t138 = t123 * t123;
        let t140 = 1.0 / t138 / t123;
        let t143 = 8.0 * t130 * t132 + 64.0 * t137 * t140 + 1.0;
        let t144 = 1.0 / t143;
        let t145 = t129 * t144;
        let t146 = param_kappa + t110 + param_c;
        let t151 = param_kappa * (1.0 - param_kappa / t146) - t115;
        let t153 = t145 * t151 + t115 + 1.0;
        let t157 = piecewise3(t93, 0.0, -3.0 / 8.0 * t6 * t102 * t153);
        let tzk0 = t92 + t157;
        zk[ip] += tzk0;
    }
}
