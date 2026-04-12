//! GGA_C_PBE_ERF_GWS exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pbe_erf_gws.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use crate::math::piecewise::{piecewise3};
use crate::math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_pbe_erf_gws_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_a_c: f64,
    param_beta: f64,
    param_gamma: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t8 = 1.0 / t7;
        let t9 = t6 * t8;
        let t10 = t4 * t9;
        let t12 = 1.0 + 0.53425e-1 * t10;
        let t13 = f64::sqrt(t10);
        let t16 = pow_3_2(t10);
        let t18 = t1 * t1;
        let t19 = t3 * t3;
        let t20 = t18 * t19;
        let t21 = t7 * t7;
        let t22 = 1.0 / t21;
        let t23 = t5 * t22;
        let t24 = t20 * t23;
        let t26 = 0.379785e1 * t13 + 0.8969e0 * t10 + 0.204775e0 * t16 + 0.123235e0 * t24;
        let t29 = 1.0 + 0.16081824322151104822e2 / t26;
        let t30 = f64::ln(t29);
        let t32 = 0.62182e-1 * t12 * t30;
        let t33 = 1.0 <= zeta_threshold;
        let t34 = pow_1_3(zeta_threshold);
        let t36 = piecewise3(t33, t34 * zeta_threshold, 1.0);
        let t39 = M_CBRT2;
        let t43 = (2.0 * t36 - 2.0) / (2.0 * t39 - 2.0);
        let t45 = 1.0 + 0.278125e-1 * t10;
        let t50 = 0.51785e1 * t13 + 0.905775e0 * t10 + 0.1100325e0 * t16 + 0.1241775e0 * t24;
        let t53 = 1.0 + 0.29608574643216675549e2 / t50;
        let t54 = f64::ln(t53);
        let t57 = 0.19751789702565206229e-1 * t43 * t45 * t54;
        let t58 = f64::ln(2.0);
        let t59 = t58 - 1.0;
        let t60 = 2.0 * t59;
        let t62 = 0.2923025e1 * param_hyb_omega_0 * t13;
        let t64 = pow_1_3(9.0);
        let t65 = t64 * t64;
        let t73 = param_hyb_omega_0 * param_hyb_omega_0;
        let t75 = (0.344851e1 - M_PI * t5 * t65 * t3 / t59 / 12.0) * t73 * t1;
        let t76 = t3 * t6;
        let t77 = t76 * t8;
        let t80 = t73 * param_hyb_omega_0;
        let t81 = t13 * t10;
        let t84 = 1.0 + t62 + t75 * t77 / 4.0 + 0.48968e0 * t80 * t81;
        let t85 = t73 * t1;
        let t88 = 1.0 + t62 + 0.8621275e0 * t85 * t77;
        let t89 = 1.0 / t88;
        let t91 = f64::ln(t84 * t89);
        let t93 = M_PI * M_PI;
        let t94 = 1.0 / t93;
        let t96 = 1.0 / rho[ip];
        let t100 = t3 * t2;
        let t101 = t1 * t100;
        let t103 = 1.0 / t7 / rho[ip];
        let t104 = t6 * t103;
        let t107 = 1.0 + 0.5175e-2 * t10 + 0.204825e-1 * t24 - 0.30486129349252551566e-2 * t96 + 0.3485625e-3 * t101 * t104;
        let t110 = f64::exp(-0.1881e0 * t10);
        let t111 = M_SQRT2;
        let t112 = t110 * t111;
        let t116 = t18 * t19 * t94;
        let t117 = t116 * t5;
        let t119 = 1.0 / t21 / rho[ip];
        let t121 = t4 * t9 * t39;
        let t124 = (1.0 - 0.56675e-2 * t121) * t65;
        let t125 = 1.0 / t100;
        let t126 = t124 * t125;
        let t127 = t1 * t21;
        let t129 = t39 * t39;
        let t133 = 1.0 + 0.107975e0 * t121 + 0.1e-1 * t20 * t23 * t129;
        let t134 = 1.0 / t133;
        let t137 = t126 * t127 * t134 / 15.0;
        let t139 = f64::exp(-0.775e-1 * t10);
        let t142 = -0.12375e1 * t10 + t24 / 4.0;
        let t143 = t139 * t142;
        let t144 = M_PI * rho[ip];
        let t147 = t137 + 4.0 / 3.0 * t143 * t144;
        let t154 = t107 * t110;
        let t156 = t154 / 2.0 - 1.0 / 2.0;
        let t159 = t5 * t119;
        let t161 = f64::exp(-0.13675e0 * t10);
        let t164 = -0.97e-1 * t10 + 0.169e0 * t24;
        let t166 = t161 * t164 * t1;
        let t167 = 1.0 / t19;
        let t168 = t167 * t6;
        let t169 = t168 * t21;
        let t172 = t65 * t125;
        let t175 = t137 + t166 * t169 / 3.0 - t172 * t127 / 15.0;
        let t179 = -t32 + t57;
        let t184 = t73 * t73;
        let t186 = t116 * t159;
        let t187 = t184 * param_hyb_omega_0;
        let t188 = t111 * t187;
        let t189 = t154 * t188;
        let t195 = rho[ip] * rho[ip];
        let t196 = 1.0 / t195;
        let t200 = t184 * t73;
        let t203 = 1.0 / t21 / t195;
        let t205 = t184 * t184;
        let t209 = t60 * t91 * t94 + (-0.31505407223141117834e-1 * t96 * t107 * t112 - 0.53884053046145740922e-2 * t117 * t119 * t147 * t111) * t80 + (-0.83762820535504401876e-1 * t96 * t156 - 0.11938374665504764976e-1 * t116 * t159 * t175 + 0.42708890021612718669e0 * t101 * t104 * t179) * t184 - 0.11974234010254609094e-1 * t186 * t189 + (-0.31835665774679373271e-1 * t116 * t159 * t156 + 0.533250677421793803e-1 * t196 * t179) * t200 + 0.20267214298646782767e-1 * t117 * t203 * t179 * t205;
        let t213 = 1.0 + 0.15403623315025e0 * t20 * t23 * t73;
        let t214 = t213 * t213;
        let t215 = t214 * t214;
        let t216 = 1.0 / t215;
        let t217 = t209 * t216;
        let t218 = t34 * t34;
        let t219 = piecewise3(t33, t218, 1.0);
        let t220 = t219 * t219;
        let t221 = t220 * t219;
        let t222 = param_gamma * t221;
        let t223 = -t32 + t57 - t217;
        let t224 = 1.0 / t179;
        let t226 = f64::powf(t223 * t224, param_a_c);
        let t227 = param_beta * t226;
        let t228 = t227 * sigma[ip];
        let t230 = 1.0 / t7 / t195;
        let t231 = t230 * t39;
        let t232 = 1.0 / t220;
        let t233 = t231 * t232;
        let t234 = t228 * t233;
        let t235 = 1.0 / t3;
        let t236 = t18 * t235;
        let t237 = t236 * t5;
        let t238 = 1.0 / param_gamma;
        let t242 = f64::exp(-t223 / t221 * t238);
        let t243 = t242 - 1.0;
        let t244 = 1.0 / t243;
        let t245 = t238 * t244;
        let t247 = t227 * t245 * sigma[ip];
        let t250 = t247 * t233 * t237 / 96.0;
        let t251 = 1.0 + t250;
        let t252 = t238 * t251;
        let t253 = param_beta * param_beta;
        let t254 = t226 * t226;
        let t255 = t253 * t254;
        let t256 = param_gamma * param_gamma;
        let t257 = 1.0 / t256;
        let t258 = t243 * t243;
        let t259 = 1.0 / t258;
        let t260 = t257 * t259;
        let t261 = sigma[ip] * sigma[ip];
        let t263 = t255 * t260 * t261;
        let t264 = t195 * t195;
        let t266 = 1.0 / t21 / t264;
        let t267 = t266 * t129;
        let t268 = t220 * t220;
        let t269 = 1.0 / t268;
        let t270 = t267 * t269;
        let t271 = t1 * t167;
        let t272 = t271 * t6;
        let t273 = t270 * t272;
        let t276 = 1.0 + t250 + t263 * t273 / 3072.0;
        let t277 = 1.0 / t276;
        let t278 = t252 * t277;
        let t279 = t237 * t278;
        let t282 = 1.0 + t234 * t279 / 96.0;
        let t283 = f64::ln(t282);
        let t284 = t222 * t283;
        let tzk0 = -t32 + t57 - t217 + t284;
        zk[ip] += tzk0;
    }
}
