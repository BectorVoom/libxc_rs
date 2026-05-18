//! MGGA_K_PC07 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_pc07.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_k_pc07_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = t4 * t4;
        let t6 = M_CBRTPI;
        let t8 = t5 * t6 * M_PI;
        let t9 = 1.0 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t12 = piecewise5::<f64>(t9, t10, t9, -t10, 0.0);
        let t13 = 1.0 + t12;
        let t15 = pow_1_3::<f64>(zeta_threshold);
        let t16 = t15 * t15;
        let t18 = pow_1_3::<f64>(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3::<f64>(t13 <= zeta_threshold, t16 * zeta_threshold, t19 * t13);
        let t22 = pow_1_3::<f64>(rho[ip]);
        let t23 = t22 * t22;
        let t24 = t21 * t23;
        let t25 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3::<f64>(t26);
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t30 = t25 * t29;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t23 / t34;
        let t38 = t30 * t33 * t36;
        let t39 = 5.0 / 72.0 * t38;
        let t41 = lapl[ip] * t32;
        let t43 = 1.0 / t23 / rho[ip];
        let t47 = t25 * t25;
        let t49 = 1.0 / t27 / t26;
        let t50 = t47 * t49;
        let t51 = lapl[ip] * lapl[ip];
        let t52 = t51 * t31;
        let t53 = t34 * rho[ip];
        let t55 = 1.0 / t22 / t53;
        let t58 = t50 * t52 * t55 / 2916.0;
        let t59 = t50 * sigma[ip];
        let t60 = t34 * t34;
        let t62 = 1.0 / t22 / t60;
        let t63 = t31 * t62;
        let t64 = t63 * lapl[ip];
        let t66 = t59 * t64 / 2592.0;
        let t67 = sigma[ip] * sigma[ip];
        let t68 = t67 * t31;
        let t69 = t60 * rho[ip];
        let t71 = 1.0 / t22 / t69;
        let t74 = t50 * t68 * t71 / 8748.0;
        let t75 = 1.0 + 5.0 / 648.0 * t38 + 5.0 / 54.0 * t30 * t41 * t43 + t58 - t66 + t74;
        let t76 = t58 - t66 + t74;
        let t77 = t76 * t76;
        let t78 = 1.0 + t39;
        let t79 = t78 * t78;
        let t80 = 1.0 / t79;
        let t82 = t77 * t80 + 1.0;
        let t83 = f64::sqrt(t82);
        let t84 = 1.0 / t83;
        let t86 = t75 * t84 - t39;
        let t87 = param_a / 40.0;
        let t88 = t86 <= t87;
        let t89 = 39.0 / 40.0 * param_a;
        let t90 = t89 <= t86;
        let t91 = param_a * param_b;
        let t92 = t86 < t87;
        let t93 = piecewise3::<f64>(t92, t87, t86);
        let t94 = t93 < t89;
        let t95 = piecewise3::<f64>(t94, t93, t89);
        let t96 = 1.0 / t95;
        let t98 = f64::exp(-t91 * t96);
        let t99 = param_a - t95;
        let t102 = f64::exp(-param_a / t99);
        let t103 = 1.0 + t102;
        let t104 = f64::powf(t103, param_b);
        let t105 = t98 * t104;
        let t107 = f64::exp(-param_a * t96);
        let t108 = t107 + t102;
        let t109 = f64::powf(t108, param_b);
        let t110 = 1.0 / t109;
        let t111 = t105 * t110;
        let t112 = piecewise5::<f64>(t88, 0.0, t90, 1.0, t111);
        let t114 = t86 * t112 + t39;
        let t118 = piecewise3::<f64>(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t114);
        let tzk0 = 2.0 * t118;
        zk[ip] += tzk0;
        let t120 = t21 / t22;
        let t125 = 1.0 / t23 / t53;
        let t126 = t33 * t125;
        let t127 = t30 * t126;
        let t128 = 5.0 / 27.0 * t127;
        let t135 = 5.0 / 4374.0 * t50 * t52 * t62;
        let t136 = t31 * t71;
        let t137 = t136 * lapl[ip];
        let t139 = 13.0 / 7776.0 * t59 * t137;
        let t140 = t60 * t34;
        let t142 = 1.0 / t22 / t140;
        let t145 = 4.0 / 6561.0 * t50 * t68 * t142;
        let t146 = -5.0 / 243.0 * t127 - 25.0 / 162.0 * t30 * t41 * t36 - t135 + t139 - t145;
        let t149 = 1.0 / t83 / t82;
        let t150 = t75 * t149;
        let t151 = t76 * t80;
        let t152 = -t135 + t139 - t145;
        let t155 = t79 * t78;
        let t156 = 1.0 / t155;
        let t158 = t77 * t156 * t25;
        let t159 = t29 * sigma[ip];
        let t160 = t32 * t125;
        let t161 = t159 * t160;
        let t164 = 2.0 * t151 * t152 + 10.0 / 27.0 * t158 * t161;
        let t167 = t146 * t84 - t150 * t164 / 2.0 + t128;
        let t169 = t95 * t95;
        let t170 = 1.0 / t169;
        let t171 = t91 * t170;
        let t172 = piecewise3::<f64>(t92, 0.0, t167);
        let t173 = piecewise3::<f64>(t94, t172, 0.0);
        let t174 = t173 * t98;
        let t175 = t104 * t110;
        let t176 = t174 * t175;
        let t178 = t105 * t91;
        let t179 = t99 * t99;
        let t180 = 1.0 / t179;
        let t181 = t180 * t173;
        let t182 = 1.0 / t103;
        let t184 = t102 * t182 * t110;
        let t187 = param_a * t170;
        let t188 = t173 * t107;
        let t190 = param_a * t180;
        let t191 = t173 * t102;
        let t193 = t187 * t188 - t190 * t191;
        let t195 = 1.0 / t108;
        let t199 = piecewise5::<f64>(t88, 0.0, t90, 0.0, -t111 * param_b * t193 * t195 - t178 * t181 * t184 + t171 * t176);
        let t201 = t167 * t112 + t86 * t199 - t128;
        let t206 = piecewise3::<f64>(t3, 0.0, t8 * t120 * t114 / 10.0 + 3.0 / 20.0 * t8 * t24 * t201);
        let tvrho0 = 2.0 * rho[ip] * t206 + 2.0 * t118;
        vrho[ip] += tvrho0;
        let t209 = t32 * t36;
        let t210 = t30 * t209;
        let t211 = 5.0 / 72.0 * t210;
        let t213 = t50 * t64;
        let t214 = t213 / 2592.0;
        let t215 = sigma[ip] * t31;
        let t217 = t50 * t215 * t71;
        let t218 = t217 / 4374.0;
        let t219 = 5.0 / 648.0 * t210 - t214 + t218;
        let t221 = -t214 + t218;
        let t224 = t29 * t32;
        let t225 = t224 * t36;
        let t228 = 2.0 * t151 * t221 - 5.0 / 36.0 * t158 * t225;
        let t231 = t219 * t84 - t150 * t228 / 2.0 - t211;
        let t233 = piecewise3::<f64>(t92, 0.0, t231);
        let t234 = piecewise3::<f64>(t94, t233, 0.0);
        let t235 = t234 * t98;
        let t236 = t235 * t175;
        let t238 = t180 * t234;
        let t241 = t234 * t107;
        let t243 = t234 * t102;
        let t245 = t187 * t241 - t190 * t243;
        let t246 = param_b * t245;
        let t250 = piecewise5::<f64>(t88, 0.0, t90, 0.0, -t111 * t246 * t195 - t178 * t238 * t184 + t171 * t236);
        let t252 = t231 * t112 + t86 * t250 + t211;
        let t256 = piecewise3::<f64>(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t252);
        let tvsigma0 = 2.0 * rho[ip] * t256;
        vsigma[ip] += tvsigma0;
        let t264 = t50 * lapl[ip] * t31 * t55 / 1458.0;
        let t267 = t50 * t215 * t62 / 2592.0;
        let t268 = 5.0 / 54.0 * t30 * t32 * t43 + t264 - t267;
        let t270 = t264 - t267;
        let t271 = t151 * t270;
        let t273 = -t150 * t271 + t268 * t84;
        let t275 = piecewise3::<f64>(t92, 0.0, t273);
        let t276 = piecewise3::<f64>(t94, t275, 0.0);
        let t277 = t276 * t98;
        let t278 = t277 * t175;
        let t280 = t180 * t276;
        let t283 = t276 * t107;
        let t285 = t276 * t102;
        let t287 = t187 * t283 - t190 * t285;
        let t288 = param_b * t287;
        let t292 = piecewise5::<f64>(t88, 0.0, t90, 0.0, -t111 * t288 * t195 - t178 * t280 * t184 + t171 * t278);
        let t294 = t273 * t112 + t86 * t292;
        let t298 = piecewise3::<f64>(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t294);
        let tvlapl0 = 2.0 * rho[ip] * t298;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
    }
}
