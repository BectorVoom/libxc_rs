//! MGGA_K_CSK exc pol kernel.
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
pub fn mgga_k_csk_exc_pol(
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
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = rho0 + rho1;
        let t9 = 1.0 / t8;
        let t12 = 2.0 * rho0 * t9 <= zeta_threshold;
        let t13 = zeta_threshold - 1.0;
        let t16 = 2.0 * rho1 * t9 <= zeta_threshold;
        let t17 = -t13;
        let t18 = rho0 - rho1;
        let t20 = piecewise5::<f64>(t12, t13, t16, t17, t18 * t9);
        let t21 = 1.0 + t20;
        let t22 = t21 <= zeta_threshold;
        let t23 = pow_1_3::<f64>(zeta_threshold);
        let t24 = t23 * t23;
        let t25 = t24 * zeta_threshold;
        let t26 = pow_1_3::<f64>(t21);
        let t27 = t26 * t26;
        let t29 = piecewise3::<f64>(t22, t25, t27 * t21);
        let t30 = pow_1_3::<f64>(t8);
        let t31 = t30 * t30;
        let t32 = t29 * t31;
        let t33 = M_CBRT6;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3::<f64>(t34);
        let t36 = t35 * t35;
        let t38 = t33 / t36;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3::<f64>(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t45 = t38 * sigma0 * t43;
        let t48 = 1.0 / t41 / rho0;
        let t53 = 5.0 / 54.0 * t38 * lapl0 * t48 - 5.0 / 81.0 * t45;
        let t55 = f64::ln(1.0 - f64::EPSILON);
        let t56 = 1.0 / param_csk_a;
        let t57 = f64::powf(-t55, -t56);
        let t58 = t53 < -t57;
        let t59 = f64::ln(f64::EPSILON);
        let t60 = f64::powf(-t59, -t56);
        let t61 = -t60 < t53;
        let t62 = piecewise3::<f64>(t61, -t60, t53);
        let t63 = -t57 < t62;
        let t64 = piecewise3::<f64>(t63, t62, -t57);
        let t65 = f64::abs(t64);
        let t66 = f64::powf(t65, param_csk_a);
        let t67 = 1.0 / t66;
        let t68 = f64::exp(-t67);
        let t69 = 1.0 - t68;
        let t70 = f64::powf(t69, t56);
        let t71 = piecewise5::<f64>(t58, 0.0, t61, 1.0, t70);
        let t73 = 1.0 + 5.0 / 72.0 * t45 + t53 * t71;
        let t77 = piecewise3::<f64>(t2, 0.0, 3.0 / 20.0 * t7 * t32 * t73);
        let t78 = rho1 <= dens_threshold;
        let t79 = -t18;
        let t81 = piecewise5::<f64>(t16, t13, t12, t17, t79 * t9);
        let t82 = 1.0 + t81;
        let t83 = t82 <= zeta_threshold;
        let t84 = pow_1_3::<f64>(t82);
        let t85 = t84 * t84;
        let t87 = piecewise3::<f64>(t83, t25, t85 * t82);
        let t88 = t87 * t31;
        let t89 = rho1 * rho1;
        let t90 = pow_1_3::<f64>(rho1);
        let t91 = t90 * t90;
        let t93 = 1.0 / t91 / t89;
        let t95 = t38 * sigma2 * t93;
        let t98 = 1.0 / t91 / rho1;
        let t103 = 5.0 / 54.0 * t38 * lapl1 * t98 - 5.0 / 81.0 * t95;
        let t104 = t103 < -t57;
        let t105 = -t60 < t103;
        let t106 = piecewise3::<f64>(t105, -t60, t103);
        let t107 = -t57 < t106;
        let t108 = piecewise3::<f64>(t107, t106, -t57);
        let t109 = f64::abs(t108);
        let t110 = f64::powf(t109, param_csk_a);
        let t111 = 1.0 / t110;
        let t112 = f64::exp(-t111);
        let t113 = 1.0 - t112;
        let t114 = f64::powf(t113, t56);
        let t115 = piecewise5::<f64>(t104, 0.0, t105, 1.0, t114);
        let t117 = 1.0 + 5.0 / 72.0 * t95 + t103 * t115;
        let t121 = piecewise3::<f64>(t78, 0.0, 3.0 / 20.0 * t7 * t88 * t117);
        let tzk0 = t77 + t121;
        zk[ip] += tzk0;
    }
}
