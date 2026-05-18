//! MGGA_X_FT98 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_ft98.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_ft98_vxc_unpol(
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
    param_a1: f64,
    param_a2: f64,
    param_b: f64,
    param_b1: f64,
    param_b2: f64,
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
        let t21 = param_a1 * sigma[ip];
        let t22 = M_CBRT2;
        let t23 = t22 * t22;
        let t24 = rho[ip] * rho[ip];
        let t25 = t19 * t19;
        let t27 = 1.0 / t25 / t24;
        let t28 = t23 * t27;
        let t30 = t21 * t28 + 1.0;
        let t31 = f64::sqrt(t30);
        let t32 = param_a * t31;
        let t33 = param_b1 * sigma[ip];
        let t35 = t28 * t33 + 1.0;
        let t36 = pow_1_4::<f64>(t35);
        let t37 = t36 * t36;
        let t38 = t37 * t36;
        let t39 = 1.0 / t38;
        let t40 = t32 * t39;
        let t41 = sigma[ip] * t23;
        let t42 = t41 * t27;
        let t44 = lapl[ip] * t23;
        let t46 = 1.0 / t25 / rho[ip];
        let t48 = -t44 * t46 + t42;
        let t49 = t48 * t48;
        let t50 = param_a2 * t49;
        let t51 = 1.0 + t42;
        let t52 = t51 * t51;
        let t53 = 1.0 / t52;
        let t56 = param_b * (t50 * t53 + 1.0);
        let t57 = param_b2 * param_b2;
        let t59 = f64::sqrt(t57 + 1.0);
        let t60 = t59 - param_b2;
        let t61 = sigma[ip] * sigma[ip];
        let t62 = t61 * t22;
        let t63 = t24 * t24;
        let t64 = t63 * rho[ip];
        let t66 = 1.0 / t19 / t64;
        let t67 = t62 * t66;
        let t68 = 2.0 * t67;
        let t69 = lapl[ip] * lapl[ip];
        let t70 = t69 * t22;
        let t71 = t24 * rho[ip];
        let t73 = 1.0 / t19 / t71;
        let t74 = t70 * t73;
        let t75 = 2.0 * t74;
        let t76 = t68 - t75 - param_b2;
        let t77 = pow_1_4::<f64>(f64::EPSILON);
        let t78 = 1.0 / t77;
        let t79 = t76 < -t78;
        let t85 = t76 * t76;
        let t86 = t85 * t76;
        let t87 = 1.0 / t86;
        let t89 = t85 * t85;
        let t90 = t89 * t76;
        let t91 = 1.0 / t90;
        let t96 = piecewise3::<f64>(0.0 < t76, t76, -t76);
        let t97 = t96 < t77;
        let t100 = t89 * t85;
        let t102 = t89 * t89;
        let t105 = -t78 < t76;
        let t106 = piecewise3::<f64>(t105, t76, -t78);
        let t107 = t106 * t106;
        let t108 = 1.0 + t107;
        let t109 = f64::sqrt(t108);
        let t110 = t106 + t109;
        let t112 = piecewise5::<f64>(t79, -4.0 * t67 + 4.0 * t74 + 2.0 * param_b2 - 1.0 / t76 / 2.0 + t87 / 8.0 - t91 / 16.0, t97, 1.0 - t68 + t75 + param_b2 + t85 / 2.0 - t89 / 8.0 + t100 / 16.0 - 5.0 / 128.0 * t102, 1.0 / t110);
        let t114 = t112 * t60 + 1.0;
        let t115 = t22 - 1.0;
        let t116 = t115 * t60;
        let t118 = t112 * t116 + 1.0;
        let t119 = t118 * t118;
        let t120 = t119 * t118;
        let t121 = 1.0 / t120;
        let t122 = t114 * t121;
        let t123 = t122 * t49;
        let t125 = t123 * t56 + t40 * t42 + 1.0;
        let t126 = t4 * t4;
        let t127 = 1.0 / M_PI;
        let t128 = pow_1_3::<f64>(t127);
        let t129 = t128 * t128;
        let t130 = t126 * t129;
        let t131 = M_CBRT4;
        let t133 = param_b * sigma[ip];
        let t137 = 1.0 + 81.0 / 4.0 * t130 * t131 * t133 * t28;
        let t138 = 1.0 / t137;
        let t139 = t125 * t138;
        let t140 = f64::sqrt(t139);
        let t144 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t140);
        let tzk0 = 2.0 * t144;
        zk[ip] += tzk0;
        let t145 = 1.0 / t25;
        let t146 = t18 * t145;
        let t150 = t7 * t18;
        let t151 = 1.0 / t140;
        let t152 = t19 * t151;
        let t154 = param_a / t31;
        let t155 = t154 * t39;
        let t156 = t63 * t24;
        let t158 = 1.0 / t19 / t156;
        let t159 = t158 * param_a1;
        let t164 = 1.0 / t38 / t35;
        let t165 = t32 * t164;
        let t166 = t158 * param_b1;
        let t171 = 1.0 / t25 / t71;
        let t172 = t41 * t171;
        let t175 = param_a2 * t48;
        let t179 = -8.0 / 3.0 * t172 + 5.0 / 3.0 * t44 * t27;
        let t184 = 1.0 / t52 / t51;
        let t185 = t50 * t184;
        let t189 = param_b * (2.0 * t175 * t53 * t179 + 16.0 / 3.0 * t185 * t172);
        let t191 = t56 * t60;
        let t192 = t62 * t158;
        let t195 = 1.0 / t19 / t63;
        let t196 = t70 * t195;
        let t198 = 1.0 / t85;
        let t199 = 32.0 / 3.0 * t192;
        let t200 = 20.0 / 3.0 * t196;
        let t201 = -t199 + t200;
        let t204 = 1.0 / t89;
        let t207 = 1.0 / t100;
        let t216 = t89 * t86;
        let t220 = t110 * t110;
        let t221 = 1.0 / t220;
        let t222 = piecewise3::<f64>(t105, t201, 0.0);
        let t223 = 1.0 / t109;
        let t224 = t223 * t106;
        let t226 = t222 * t224 + t222;
        let t228 = piecewise5::<f64>(t79, 64.0 / 3.0 * t192 - 40.0 / 3.0 * t196 + t198 * t201 / 2.0 - 3.0 / 8.0 * t204 * t201 + 5.0 / 16.0 * t207 * t201, t97, t199 - t200 + t76 * t201 - t86 * t201 / 2.0 + 3.0 / 8.0 * t90 * t201 - 5.0 / 16.0 * t216 * t201, -t221 * t226);
        let t229 = t228 * t121;
        let t230 = t229 * t49;
        let t232 = t119 * t119;
        let t233 = 1.0 / t232;
        let t234 = t114 * t233;
        let t235 = t56 * t234;
        let t236 = t49 * t115;
        let t237 = t60 * t228;
        let t238 = t236 * t237;
        let t241 = t56 * t114;
        let t242 = t121 * t48;
        let t243 = t242 * t179;
        let t246 = -8.0 / 3.0 * t155 * t62 * t159 + 4.0 * t165 * t62 * t166 - 8.0 / 3.0 * t40 * t172 + t189 * t123 + t191 * t230 - 3.0 * t235 * t238 + 2.0 * t241 * t243;
        let t248 = t137 * t137;
        let t249 = 1.0 / t248;
        let t251 = t125 * t249 * t130;
        let t252 = t131 * param_b;
        let t253 = t252 * t172;
        let t256 = t138 * t246 + 54.0 * t251 * t253;
        let t261 = piecewise3::<f64>(t3, 0.0, -t7 * t146 * t140 / 8.0 - 3.0 / 16.0 * t150 * t152 * t256);
        let tvrho0 = 2.0 * rho[ip] * t261 + 2.0 * t144;
        vrho[ip] += tvrho0;
        let t264 = sigma[ip] * t22;
        let t272 = t39 * t23;
        let t275 = t53 * t23;
        let t276 = t275 * t27;
        let t278 = t184 * t23;
        let t279 = t278 * t27;
        let t283 = param_b * (2.0 * t175 * t276 - 2.0 * t279 * t50);
        let t285 = t264 * t66;
        let t287 = t198 * sigma[ip];
        let t288 = t22 * t66;
        let t291 = t204 * sigma[ip];
        let t294 = t207 * sigma[ip];
        let t298 = 4.0 * t285;
        let t299 = t76 * sigma[ip];
        let t302 = t86 * sigma[ip];
        let t305 = t90 * sigma[ip];
        let t308 = t216 * sigma[ip];
        let t312 = piecewise3::<f64>(t105, t298, 0.0);
        let t314 = t224 * t312 + t312;
        let t316 = piecewise5::<f64>(t79, -8.0 * t285 + 2.0 * t287 * t288 - 3.0 / 2.0 * t291 * t288 + 5.0 / 4.0 * t294 * t288, t97, -t298 + 4.0 * t299 * t288 - 2.0 * t302 * t288 + 3.0 / 2.0 * t305 * t288 - 5.0 / 4.0 * t308 * t288, -t221 * t314);
        let t317 = t316 * t121;
        let t318 = t317 * t49;
        let t320 = t60 * t316;
        let t321 = t236 * t320;
        let t324 = t242 * t28;
        let t325 = t241 * t324;
        let t327 = t155 * t264 * t66 * param_a1 - 3.0 / 2.0 * t165 * t264 * t66 * param_b1 + t32 * t272 * t27 + t283 * t123 + t191 * t318 - 3.0 * t235 * t321 + 2.0 * t325;
        let t329 = t252 * t28;
        let t332 = t327 * t138 - 81.0 / 4.0 * t251 * t329;
        let t336 = piecewise3::<f64>(t3, 0.0, -3.0 / 16.0 * t150 * t152 * t332);
        let tvsigma0 = 2.0 * rho[ip] * t336;
        vsigma[ip] += tvsigma0;
        let t338 = param_b * param_a2;
        let t339 = t49 * t48;
        let t340 = t339 * t53;
        let t341 = t338 * t340;
        let t342 = t23 * t46;
        let t343 = t342 * t122;
        let t346 = lapl[ip] * t22;
        let t347 = t346 * t73;
        let t349 = t198 * lapl[ip];
        let t350 = t22 * t73;
        let t353 = t204 * lapl[ip];
        let t356 = t207 * lapl[ip];
        let t360 = 4.0 * t347;
        let t361 = t76 * lapl[ip];
        let t364 = t86 * lapl[ip];
        let t367 = t90 * lapl[ip];
        let t370 = t216 * lapl[ip];
        let t374 = piecewise3::<f64>(t105, -t360, 0.0);
        let t376 = t224 * t374 + t374;
        let t378 = piecewise5::<f64>(t79, 8.0 * t347 - 2.0 * t349 * t350 + 3.0 / 2.0 * t353 * t350 - 5.0 / 4.0 * t356 * t350, t97, t360 - 4.0 * t361 * t350 + 2.0 * t364 * t350 - 3.0 / 2.0 * t367 * t350 + 5.0 / 4.0 * t370 * t350, -t221 * t376);
        let t379 = t378 * t121;
        let t380 = t379 * t49;
        let t382 = t60 * t378;
        let t383 = t236 * t382;
        let t386 = t242 * t342;
        let t389 = t191 * t380 - 3.0 * t235 * t383 - 2.0 * t241 * t386 - 2.0 * t341 * t343;
        let t390 = t389 * t138;
        let t394 = piecewise3::<f64>(t3, 0.0, -3.0 / 16.0 * t150 * t152 * t390);
        let tvlapl0 = 2.0 * rho[ip] * t394;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
    }
}
