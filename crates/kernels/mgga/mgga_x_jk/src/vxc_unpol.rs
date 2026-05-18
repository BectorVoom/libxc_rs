//! MGGA_X_JK vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_jk.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_jk_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t6 = 1.0 / t5;
        let t7 = t4 * t6;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5::<f64>(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3::<f64>(zeta_threshold);
        let t16 = pow_1_3::<f64>(t12);
        let t18 = piecewise3::<f64>(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3::<f64>(rho[ip]);
        let t20 = t18 * t19;
        let t21 = t4 * t4;
        let t22 = param_beta * t21;
        let t24 = pow_1_3::<f64>(1.0 / M_PI);
        let t25 = 1.0 / t24;
        let t26 = M_CBRT4;
        let t27 = t25 * t26;
        let t28 = t22 * t27;
        let t29 = M_CBRT2;
        let t30 = t29 * t29;
        let t31 = sigma[ip] * t30;
        let t32 = rho[ip] * rho[ip];
        let t33 = t19 * t19;
        let t34 = t33 * t32;
        let t35 = 1.0 / t34;
        let t36 = param_gamma * param_beta;
        let t37 = f64::sqrt(sigma[ip]);
        let t38 = t36 * t37;
        let t40 = 1.0 / t19 / rho[ip];
        let t41 = t29 * t40;
        let t44 = f64::ln(t37 * t29 * t40 + f64::sqrt(pow_2::<f64>(t37 * t29 * t40) + 1.0));
        let t45 = t41 * t44;
        let t47 = t38 * t45 + 1.0;
        let t48 = 1.0 / t47;
        let t49 = t35 * t48;
        let t50 = t31 * t35;
        let t51 = lapl[ip] * t30;
        let t52 = t33 * rho[ip];
        let t53 = 1.0 / t52;
        let t55 = -t51 * t53 + t50;
        let t56 = 1.0 / sigma[ip];
        let t57 = t55 * t56;
        let t58 = t29 * t34;
        let t60 = t57 * t58 + 1.0;
        let t61 = 1.0 / t60;
        let t66 = 1.0 + 2.0 / 9.0 * t28 * t31 * t49 * t61;
        let t70 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t66);
        let tzk0 = 2.0 * t70;
        zk[ip] += tzk0;
        let t72 = t18 / t33;
        let t76 = t32 * rho[ip];
        let t78 = 1.0 / t33 / t76;
        let t79 = t78 * t48;
        let t85 = t22 * t27 * sigma[ip];
        let t86 = t30 * t35;
        let t87 = t47 * t47;
        let t88 = 1.0 / t87;
        let t89 = t88 * t61;
        let t91 = 1.0 / t19 / t32;
        let t93 = t29 * t91 * t44;
        let t95 = t36 * sigma[ip];
        let t96 = t30 * t78;
        let t97 = t50 + 1.0;
        let t98 = f64::sqrt(t97);
        let t99 = 1.0 / t98;
        let t100 = t96 * t99;
        let t103 = -4.0 / 3.0 * t95 * t100 - 4.0 / 3.0 * t38 * t93;
        let t104 = t89 * t103;
        let t105 = t86 * t104;
        let t108 = t60 * t60;
        let t109 = 1.0 / t108;
        let t110 = t48 * t109;
        let t115 = -8.0 / 3.0 * t31 * t78 + 5.0 / 3.0 * t51 * t35;
        let t116 = t115 * t56;
        let t118 = t29 * t52;
        let t121 = t116 * t58 + 8.0 / 3.0 * t57 * t118;
        let t122 = t110 * t121;
        let t123 = t86 * t122;
        let t126 = -16.0 / 27.0 * t28 * t31 * t79 * t61 - 2.0 / 9.0 * t85 * t105 - 2.0 / 9.0 * t85 * t123;
        let t131 = piecewise3::<f64>(t3, 0.0, -t7 * t72 * t66 / 8.0 - 3.0 / 8.0 * t7 * t20 * t126);
        let tvrho0 = 2.0 * rho[ip] * t131 + 2.0 * t70;
        vrho[ip] += tvrho0;
        let t134 = t48 * t61;
        let t138 = t36 / t37;
        let t140 = t86 * t99;
        let t143 = t138 * t45 / 2.0 + t36 * t140 / 2.0;
        let t144 = t89 * t143;
        let t145 = t86 * t144;
        let t148 = sigma[ip] * sigma[ip];
        let t149 = 1.0 / t148;
        let t150 = t55 * t149;
        let t152 = -t150 * t58 + 2.0 * t56;
        let t153 = t110 * t152;
        let t154 = t86 * t153;
        let t157 = 2.0 / 9.0 * t28 * t86 * t134 - 2.0 / 9.0 * t85 * t145 - 2.0 / 9.0 * t85 * t154;
        let t161 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t157);
        let tvsigma0 = 2.0 * rho[ip] * t161;
        vsigma[ip] += tvsigma0;
        let t163 = t6 * t18;
        let t164 = t40 * param_beta;
        let t166 = t30 * t48;
        let t168 = t27 * t166 * t109;
        let t171 = piecewise3::<f64>(t3, 0.0, -t163 * t164 * t168 / 2.0);
        let tvlapl0 = 2.0 * rho[ip] * t171;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
    }
}
