//! MGGA_X_VT84 exc pol kernel.
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
pub fn mgga_x_vt84_exc_pol(
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
        let t19 = piecewise5::<f64>(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3::<f64>(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3::<f64>(t20);
        let t26 = piecewise3::<f64>(t21, t23, t24 * t20);
        let t27 = pow_1_3::<f64>(t7);
        let t28 = t26 * t27;
        let t29 = sigma0 * sigma0;
        let t30 = t29 * sigma0;
        let t31 = rho0 * rho0;
        let t32 = t31 * rho0;
        let t33 = 1.0 / t32;
        let t34 = t30 * t33;
        let t35 = tau0 * tau0;
        let t36 = t35 * tau0;
        let t37 = 1.0 / t36;
        let t38 = 1.0 / t31;
        let t39 = t29 * t38;
        let t40 = 1.0 / t35;
        let t41 = t39 * t40;
        let t43 = 1.0 + t41 / 64.0;
        let t44 = t43 * t43;
        let t45 = 1.0 / t44;
        let t46 = t37 * t45;
        let t50 = M_CBRT6;
        let t51 = (10.0 / 81.0 + 0.419826171875e-2 * t34 * t46) * t50;
        let t52 = M_PI * M_PI;
        let t53 = pow_1_3::<f64>(t52);
        let t54 = t53 * t53;
        let t55 = 1.0 / t54;
        let t56 = t55 * sigma0;
        let t57 = pow_1_3::<f64>(rho0);
        let t58 = t57 * t57;
        let t60 = 1.0 / t58 / t31;
        let t61 = t56 * t60;
        let t65 = 1.0 / t58 / rho0;
        let t67 = sigma0 * t60;
        let t69 = tau0 * t65 - t67 / 8.0;
        let t70 = t69 * t50;
        let t73 = 5.0 / 9.0 * t70 * t55 - 1.0;
        let t74 = t55 * t73;
        let t77 = 1.0 + 0.22222222222222222222e0 * t70 * t74;
        let t78 = f64::sqrt(t77);
        let t79 = 1.0 / t78;
        let t82 = t50 * t55;
        let t83 = t82 * t67;
        let t85 = 9.0 / 20.0 * t73 * t79 + t83 / 36.0;
        let t86 = t85 * t85;
        let t89 = t50 * t50;
        let t91 = 1.0 / t53 / t52;
        let t92 = t89 * t91;
        let t93 = t31 * t31;
        let t94 = t93 * rho0;
        let t96 = 1.0 / t57 / t94;
        let t98 = t92 * t29 * t96;
        let t100 = 162.0 * t41 + 50.0 * t98;
        let t101 = f64::sqrt(t100);
        let t106 = t93 * t93;
        let t107 = 1.0 / t106;
        let t110 = t51 * t61 / 24.0 + 146.0 / 2025.0 * t86 - 73.0 / 97200.0 * t85 * t101 + 0.26505934954444613795e-4 * t98 + 0.19577914932045745128e-2 * t41 + 0.10930269815274441669e-5 * t30 * t107;
        let t112 = 1.0 + 0.58733744796137235383e-1 * t83;
        let t113 = t112 * t112;
        let t114 = 1.0 / t113;
        let t115 = t110 * t114;
        let t117 = f64::exp(-0.1863e-3 * t115);
        let t118 = 1.0 + t115;
        let t119 = 1.0 / t118;
        let t120 = t117 * t119;
        let t122 = t110 * t110;
        let t123 = t113 * t113;
        let t124 = 1.0 / t123;
        let t127 = f64::exp(-0.150903e-2 * t122 * t124);
        let t128 = 1.0 - t127;
        let t129 = 1.0 / t110;
        let t132 = 10.0 / 81.0 * t129 * t113 - 1.0;
        let t134 = t115 * t120 + t128 * t132 + 1.0;
        let t138 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t134);
        let t139 = rho1 <= dens_threshold;
        let t140 = -t17;
        let t142 = piecewise5::<f64>(t15, t12, t11, t16, t140 * t8);
        let t143 = 1.0 + t142;
        let t144 = t143 <= zeta_threshold;
        let t145 = pow_1_3::<f64>(t143);
        let t147 = piecewise3::<f64>(t144, t23, t145 * t143);
        let t148 = t147 * t27;
        let t149 = sigma2 * sigma2;
        let t150 = t149 * sigma2;
        let t151 = rho1 * rho1;
        let t152 = t151 * rho1;
        let t153 = 1.0 / t152;
        let t154 = t150 * t153;
        let t155 = tau1 * tau1;
        let t156 = t155 * tau1;
        let t157 = 1.0 / t156;
        let t158 = 1.0 / t151;
        let t159 = t149 * t158;
        let t160 = 1.0 / t155;
        let t161 = t159 * t160;
        let t163 = 1.0 + t161 / 64.0;
        let t164 = t163 * t163;
        let t165 = 1.0 / t164;
        let t166 = t157 * t165;
        let t170 = (10.0 / 81.0 + 0.419826171875e-2 * t154 * t166) * t50;
        let t171 = t55 * sigma2;
        let t172 = pow_1_3::<f64>(rho1);
        let t173 = t172 * t172;
        let t175 = 1.0 / t173 / t151;
        let t176 = t171 * t175;
        let t180 = 1.0 / t173 / rho1;
        let t182 = sigma2 * t175;
        let t184 = tau1 * t180 - t182 / 8.0;
        let t185 = t184 * t50;
        let t188 = 5.0 / 9.0 * t185 * t55 - 1.0;
        let t189 = t55 * t188;
        let t192 = 1.0 + 0.22222222222222222222e0 * t185 * t189;
        let t193 = f64::sqrt(t192);
        let t194 = 1.0 / t193;
        let t197 = t82 * t182;
        let t199 = 9.0 / 20.0 * t188 * t194 + t197 / 36.0;
        let t200 = t199 * t199;
        let t203 = t151 * t151;
        let t204 = t203 * rho1;
        let t206 = 1.0 / t172 / t204;
        let t208 = t92 * t149 * t206;
        let t210 = 162.0 * t161 + 50.0 * t208;
        let t211 = f64::sqrt(t210);
        let t216 = t203 * t203;
        let t217 = 1.0 / t216;
        let t220 = t170 * t176 / 24.0 + 146.0 / 2025.0 * t200 - 73.0 / 97200.0 * t199 * t211 + 0.26505934954444613795e-4 * t208 + 0.19577914932045745128e-2 * t161 + 0.10930269815274441669e-5 * t150 * t217;
        let t222 = 1.0 + 0.58733744796137235383e-1 * t197;
        let t223 = t222 * t222;
        let t224 = 1.0 / t223;
        let t225 = t220 * t224;
        let t227 = f64::exp(-0.1863e-3 * t225);
        let t228 = 1.0 + t225;
        let t229 = 1.0 / t228;
        let t230 = t227 * t229;
        let t232 = t220 * t220;
        let t233 = t223 * t223;
        let t234 = 1.0 / t233;
        let t237 = f64::exp(-0.150903e-2 * t232 * t234);
        let t238 = 1.0 - t237;
        let t239 = 1.0 / t220;
        let t242 = 10.0 / 81.0 * t239 * t223 - 1.0;
        let t244 = t225 * t230 + t238 * t242 + 1.0;
        let t248 = piecewise3::<f64>(t139, 0.0, -3.0 / 8.0 * t6 * t148 * t244);
        let tzk0 = t138 + t248;
        zk[ip] += tzk0;
    }
}
