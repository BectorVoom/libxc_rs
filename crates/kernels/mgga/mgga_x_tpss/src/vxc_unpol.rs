//! MGGA_X_TPSS vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tpss.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_tpss_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_BLOC_a: f64,
    param_BLOC_b: f64,
    param_b: f64,
    param_c: f64,
    param_e: f64,
    param_kappa: f64,
    param_mu: f64,
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
        let t11 = piecewise5::<f64>(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3::<f64>(zeta_threshold);
        let t16 = pow_1_3::<f64>(t12);
        let t18 = piecewise3::<f64>(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3::<f64>(rho[ip]);
        let t20 = t18 * t19;
        let t21 = 1.0 / rho[ip];
        let t23 = 1.0 / tau[ip];
        let t25 = sigma[ip] * t21 * t23 / 8.0;
        let t26 = param_BLOC_b * sigma[ip];
        let t30 = param_BLOC_a + t26 * t21 * t23 / 8.0;
        let t31 = f64::powf(t25, t30);
        let t32 = param_c * t31;
        let t33 = sigma[ip] * sigma[ip];
        let t34 = rho[ip] * rho[ip];
        let t35 = 1.0 / t34;
        let t36 = t33 * t35;
        let t37 = tau[ip] * tau[ip];
        let t38 = 1.0 / t37;
        let t39 = t36 * t38;
        let t41 = 1.0 + t39 / 64.0;
        let t42 = t41 * t41;
        let t43 = 1.0 / t42;
        let t46 = M_CBRT6;
        let t47 = (10.0 / 81.0 + t32 * t43) * t46;
        let t48 = M_PI * M_PI;
        let t49 = pow_1_3::<f64>(t48);
        let t50 = t49 * t49;
        let t51 = 1.0 / t50;
        let t52 = t47 * t51;
        let t53 = M_CBRT2;
        let t54 = t53 * t53;
        let t55 = sigma[ip] * t54;
        let t56 = t19 * t19;
        let t58 = 1.0 / t56 / t34;
        let t59 = t55 * t58;
        let t62 = tau[ip] * t54;
        let t64 = 1.0 / t56 / rho[ip];
        let t67 = t62 * t64 - t59 / 8.0;
        let t71 = 5.0 / 9.0 * t67 * t46 * t51 - 1.0;
        let t72 = param_b * t67;
        let t73 = t46 * t51;
        let t74 = t73 * t71;
        let t77 = 5.0 * t72 * t74 + 9.0;
        let t78 = f64::sqrt(t77);
        let t79 = 1.0 / t78;
        let t84 = 27.0 / 20.0 * t71 * t79 + t73 * t59 / 36.0;
        let t85 = t84 * t84;
        let t88 = t46 * t46;
        let t90 = 1.0 / t49 / t48;
        let t91 = t88 * t90;
        let t92 = t33 * t53;
        let t93 = t34 * t34;
        let t94 = t93 * rho[ip];
        let t96 = 1.0 / t19 / t94;
        let t97 = t92 * t96;
        let t100 = 100.0 * t91 * t97 + 162.0 * t39;
        let t101 = f64::sqrt(t100);
        let t105 = 1.0 / param_kappa * t88;
        let t106 = t105 * t90;
        let t109 = f64::sqrt(param_e);
        let t110 = t109 * t33;
        let t111 = t35 * t38;
        let t114 = param_e * param_mu;
        let t115 = t48 * t48;
        let t116 = 1.0 / t115;
        let t117 = t33 * sigma[ip];
        let t118 = t116 * t117;
        let t119 = t93 * t93;
        let t120 = 1.0 / t119;
        let t124 = t52 * t59 / 24.0 + 146.0 / 2025.0 * t85 - 73.0 / 97200.0 * t84 * t101 + 25.0 / 472392.0 * t106 * t97 + t110 * t111 / 720.0 + t114 * t118 * t120 / 576.0;
        let t125 = t109 * t46;
        let t129 = 1.0 + t125 * t51 * t59 / 24.0;
        let t130 = t129 * t129;
        let t131 = 1.0 / t130;
        let t133 = t124 * t131 + param_kappa;
        let t138 = 1.0 + param_kappa * (1.0 - param_kappa / t133);
        let t142 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t138);
        let tzk0 = 2.0 * t142;
        zk[ip] += tzk0;
        let t143 = 1.0 / t56;
        let t144 = t18 * t143;
        let t148 = t7 * t18;
        let t149 = param_kappa * param_kappa;
        let t150 = t19 * t149;
        let t151 = t133 * t133;
        let t152 = 1.0 / t151;
        let t153 = t35 * t23;
        let t154 = f64::ln(t25);
        let t159 = -t26 * t153 * t154 / 8.0 - t30 * t21;
        let t160 = t159 * t43;
        let t163 = 1.0 / t42 / t41;
        let t164 = t32 * t163;
        let t165 = t34 * rho[ip];
        let t166 = 1.0 / t165;
        let t167 = t33 * t166;
        let t168 = t167 * t38;
        let t172 = (t32 * t160 + t164 * t168 / 16.0) * t46;
        let t173 = t172 * t51;
        let t177 = 1.0 / t56 / t165;
        let t178 = t55 * t177;
        let t184 = -5.0 / 3.0 * t62 * t58 + t178 / 3.0;
        let t185 = t184 * t46;
        let t186 = t51 * t79;
        let t190 = 1.0 / t78 / t77;
        let t191 = t71 * t190;
        let t195 = t91 * t184;
        let t198 = 5.0 * param_b * t184 * t74 + 25.0 / 9.0 * t72 * t195;
        let t203 = 3.0 / 4.0 * t185 * t186 - 27.0 / 40.0 * t191 * t198 - 2.0 / 27.0 * t73 * t178;
        let t208 = 1.0 / t101;
        let t209 = t84 * t208;
        let t211 = t93 * t34;
        let t213 = 1.0 / t19 / t211;
        let t214 = t92 * t213;
        let t217 = -324.0 * t168 - 1600.0 / 3.0 * t91 * t214;
        let t222 = t166 * t38;
        let t225 = t119 * rho[ip];
        let t226 = 1.0 / t225;
        let t230 = t173 * t59 / 24.0 - t52 * t178 / 9.0 + 292.0 / 2025.0 * t84 * t203 - 73.0 / 97200.0 * t203 * t101 - 73.0 / 194400.0 * t209 * t217 - 50.0 / 177147.0 * t106 * t214 - t110 * t222 / 360.0 - t114 * t118 * t226 / 72.0;
        let t232 = t130 * t129;
        let t233 = 1.0 / t232;
        let t234 = t124 * t233;
        let t235 = t234 * t125;
        let t236 = t51 * sigma[ip];
        let t237 = t54 * t177;
        let t238 = t236 * t237;
        let t241 = t230 * t131 + 2.0 / 9.0 * t235 * t238;
        let t242 = t152 * t241;
        let t247 = piecewise3::<f64>(t3, 0.0, -t7 * t144 * t138 / 8.0 - 3.0 / 8.0 * t148 * t150 * t242);
        let tvrho0 = 2.0 * rho[ip] * t247 + 2.0 * t142;
        vrho[ip] += tvrho0;
        let t250 = param_BLOC_b * t21;
        let t251 = t23 * t154;
        let t254 = 1.0 / sigma[ip];
        let t256 = t250 * t251 / 8.0 + t30 * t254;
        let t257 = t256 * t43;
        let t258 = t32 * t257;
        let t259 = sigma[ip] * t35;
        let t260 = t259 * t38;
        let t264 = (t258 - t164 * t260 / 16.0) * t46;
        let t265 = t264 * t51;
        let t268 = t51 * t54;
        let t269 = t268 * t58;
        let t272 = t54 * t58;
        let t273 = t73 * t79;
        let t274 = t272 * t273;
        let t276 = param_b * t54;
        let t277 = t276 * t58;
        let t278 = t277 * t74;
        let t280 = t72 * t88;
        let t281 = t90 * t54;
        let t283 = t280 * t281 * t58;
        let t285 = -5.0 / 8.0 * t278 - 25.0 / 72.0 * t283;
        let t288 = t272 * t73;
        let t290 = -3.0 / 32.0 * t274 - 27.0 / 40.0 * t191 * t285 + t288 / 36.0;
        let t296 = sigma[ip] * t53;
        let t297 = t296 * t96;
        let t300 = 200.0 * t91 * t297 + 324.0 * t260;
        let t305 = t109 * sigma[ip];
        let t308 = t116 * t33;
        let t312 = t265 * t59 / 24.0 + t47 * t269 / 24.0 + 292.0 / 2025.0 * t84 * t290 - 73.0 / 97200.0 * t290 * t101 - 73.0 / 194400.0 * t209 * t300 + 25.0 / 236196.0 * t106 * t297 + t305 * t111 / 360.0 + t114 * t308 * t120 / 192.0;
        let t314 = t234 * t109;
        let t317 = t312 * t131 - t314 * t288 / 12.0;
        let t318 = t152 * t317;
        let t322 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t148 * t150 * t318);
        let tvsigma0 = 2.0 * rho[ip] * t322;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t324 = t7 * t20;
        let t325 = t149 * t152;
        let t326 = t21 * t38;
        let t331 = -t26 * t326 * t154 / 8.0 - t30 * t23;
        let t332 = t331 * t43;
        let t333 = t32 * t332;
        let t334 = t37 * tau[ip];
        let t335 = 1.0 / t334;
        let t336 = t36 * t335;
        let t340 = (t333 + t164 * t336 / 16.0) * t46;
        let t341 = t340 * t51;
        let t344 = t54 * t64;
        let t347 = t276 * t64;
        let t353 = 5.0 * t347 * t74 + 25.0 / 9.0 * t280 * t281 * t64;
        let t356 = 3.0 / 4.0 * t344 * t273 - 27.0 / 40.0 * t191 * t353;
        let t363 = t35 * t335;
        let t366 = t341 * t59 / 24.0 + 292.0 / 2025.0 * t84 * t356 - 73.0 / 97200.0 * t356 * t101 + 73.0 / 600.0 * t209 * t336 - t110 * t363 / 360.0;
        let t367 = t366 * t131;
        let t368 = t325 * t367;
        let t371 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t324 * t368);
        let tvtau0 = 2.0 * rho[ip] * t371;
        vtau[ip] += tvtau0;
    }
}
