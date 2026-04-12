//! GGA_X_OPTX vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_optx.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_optx_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_gamma: f64,
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
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = param_gamma * param_gamma;
        let t29 = param_b * t28;
        let t30 = sigma0 * sigma0;
        let t31 = rho0 * rho0;
        let t32 = t31 * t31;
        let t33 = t32 * rho0;
        let t34 = pow_1_3(rho0);
        let t36 = 1.0 / t34 / t33;
        let t39 = t34 * t34;
        let t43 = 1.0 + param_gamma * sigma0 / t39 / t31;
        let t44 = t43 * t43;
        let t45 = 1.0 / t44;
        let t48 = t29 * t30 * t36 * t45 + param_a;
        let t52 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t48);
        let t53 = rho1 <= dens_threshold;
        let t54 = -t16;
        let t56 = piecewise5(t14, t11, t10, t15, t54 * t7);
        let t57 = 1.0 + t56;
        let t58 = t57 <= zeta_threshold;
        let t59 = pow_1_3(t57);
        let t61 = piecewise3(t58, t22, t59 * t57);
        let t62 = t61 * t26;
        let t63 = sigma2 * sigma2;
        let t64 = rho1 * rho1;
        let t65 = t64 * t64;
        let t66 = t65 * rho1;
        let t67 = pow_1_3(rho1);
        let t69 = 1.0 / t67 / t66;
        let t72 = t67 * t67;
        let t76 = 1.0 + param_gamma * sigma2 / t72 / t64;
        let t77 = t76 * t76;
        let t78 = 1.0 / t77;
        let t81 = t29 * t63 * t69 * t78 + param_a;
        let t85 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t62 * t81);
        let tzk0 = t52 + t85;
        zk[ip] += tzk0;
        let t86 = t6 * t6;
        let t87 = 1.0 / t86;
        let t88 = t16 * t87;
        let t90 = piecewise5(t10, 0.0, t14, 0.0, t7 - t88);
        let t93 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t90);
        let t94 = t93 * t26;
        let t98 = t26 * t26;
        let t99 = 1.0 / t98;
        let t100 = t25 * t99;
        let t103 = t5 * t100 * t48 / 8.0;
        let t104 = t32 * t31;
        let t106 = 1.0 / t34 / t104;
        let t111 = param_b * t28 * param_gamma;
        let t112 = t30 * sigma0;
        let t113 = t32 * t32;
        let t114 = t113 * rho0;
        let t115 = 1.0 / t114;
        let t118 = 1.0 / t44 / t43;
        let t122 = -16.0 / 3.0 * t29 * t30 * t106 * t45 + 16.0 / 3.0 * t111 * t112 * t115 * t118;
        let t127 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t94 * t48 - t103 - 3.0 / 8.0 * t5 * t27 * t122);
        let t128 = t54 * t87;
        let t130 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t128);
        let t133 = piecewise3(t58, 0.0, 4.0 / 3.0 * t59 * t130);
        let t134 = t133 * t26;
        let t138 = t61 * t99;
        let t141 = t5 * t138 * t81 / 8.0;
        let t143 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t134 * t81 - t141);
        let tvrho0 = t52 + t85 + t6 * (t127 + t143);
        vrho[ip * 2] += tvrho0;
        let t147 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t88);
        let t150 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t147);
        let t151 = t150 * t26;
        let t156 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t151 * t48 - t103);
        let t158 = piecewise5(t14, 0.0, t10, 0.0, t7 - t128);
        let t161 = piecewise3(t58, 0.0, 4.0 / 3.0 * t59 * t158);
        let t162 = t161 * t26;
        let t166 = t65 * t64;
        let t168 = 1.0 / t67 / t166;
        let t172 = t63 * sigma2;
        let t173 = t65 * t65;
        let t174 = t173 * rho1;
        let t175 = 1.0 / t174;
        let t178 = 1.0 / t77 / t76;
        let t182 = 16.0 / 3.0 * t111 * t172 * t175 * t178 - 16.0 / 3.0 * t29 * t63 * t168 * t78;
        let t187 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t162 * t81 - t141 - 3.0 / 8.0 * t5 * t62 * t182);
        let tvrho1 = t52 + t85 + t6 * (t156 + t187);
        vrho[ip * 2 + 1] += tvrho1;
        let t193 = 1.0 / t113;
        let t198 = -2.0 * t111 * t30 * t193 * t118 + 2.0 * t29 * sigma0 * t36 * t45;
        let t202 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t198);
        let tvsigma0 = t6 * t202;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t206 = 1.0 / t173;
        let t211 = -2.0 * t111 * t63 * t206 * t178 + 2.0 * t29 * sigma2 * t69 * t78;
        let t215 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t62 * t211);
        let tvsigma2 = t6 * t215;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
