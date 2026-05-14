//! MGGA_X_M06L vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_m06l.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_m06l_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_a_0: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_a_6: f64,
    param_a_7: f64,
    param_a_8: f64,
    param_a_9: f64,
    param_a_10: f64,
    param_a_11: f64,
    param_d_0: f64,
    param_d_1: f64,
    param_d_2: f64,
    param_d_3: f64,
    param_d_4: f64,
    param_d_5: f64,
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
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t19 * t19;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t37 = 0.804e0 + 0.91464571985215458336e-2 * t26 * t34;
        let t40 = 0.1804e1 - 0.646416e0 / t37;
        let t42 = param_a_1;
        let t43 = t21 * t21;
        let t44 = t43 * t24;
        let t45 = 3.0 / 10.0 * t44;
        let t46 = tau[ip] * t28;
        let t48 = 1.0 / t31 / rho[ip];
        let t49 = t46 * t48;
        let t50 = t45 - t49;
        let t51 = t42 * t50;
        let t52 = t45 + t49;
        let t53 = 1.0 / t52;
        let t55 = param_a_2;
        let t56 = t50 * t50;
        let t57 = t55 * t56;
        let t58 = t52 * t52;
        let t59 = 1.0 / t58;
        let t61 = param_a_3;
        let t62 = t56 * t50;
        let t63 = t61 * t62;
        let t64 = t58 * t52;
        let t65 = 1.0 / t64;
        let t67 = param_a_4;
        let t68 = t56 * t56;
        let t69 = t67 * t68;
        let t70 = t58 * t58;
        let t71 = 1.0 / t70;
        let t73 = param_a_5;
        let t74 = t68 * t50;
        let t75 = t73 * t74;
        let t76 = t70 * t52;
        let t77 = 1.0 / t76;
        let t79 = param_a_6;
        let t80 = t68 * t56;
        let t81 = t79 * t80;
        let t82 = t70 * t58;
        let t83 = 1.0 / t82;
        let t85 = param_a_7;
        let t86 = t68 * t62;
        let t87 = t85 * t86;
        let t88 = t70 * t64;
        let t89 = 1.0 / t88;
        let t91 = param_a_8;
        let t92 = t68 * t68;
        let t93 = t91 * t92;
        let t94 = t70 * t70;
        let t95 = 1.0 / t94;
        let t97 = param_a_9;
        let t98 = t92 * t50;
        let t99 = t97 * t98;
        let t101 = 1.0 / t94 / t52;
        let t103 = param_a_10;
        let t104 = t92 * t56;
        let t105 = t103 * t104;
        let t107 = 1.0 / t94 / t58;
        let t109 = param_a_11;
        let t111 = t109 * t92 * t62;
        let t113 = 1.0 / t94 / t64;
        let t115 = t99 * t101 + t105 * t107 + t111 * t113 + t51 * t53 + t57 * t59 + t63 * t65 + t69 * t71 + t75 * t77 + t81 * t83 + t87 * t89 + t93 * t95 + param_a_0;
        let t117 = param_d_0;
        let t121 = 1.0 + 0.186726e-2 * t34 + 0.373452e-2 * t49 - 0.1120356e-2 * t44;
        let t124 = param_d_1;
        let t125 = t124 * sigma[ip];
        let t126 = t28 * t33;
        let t128 = param_d_2;
        let t131 = 2.0 * t49 - 3.0 / 5.0 * t44;
        let t133 = t125 * t126 + t128 * t131;
        let t134 = t121 * t121;
        let t135 = 1.0 / t134;
        let t137 = param_d_3;
        let t138 = sigma[ip] * sigma[ip];
        let t139 = t137 * t138;
        let t140 = t30 * t30;
        let t141 = t140 * rho[ip];
        let t143 = 1.0 / t19 / t141;
        let t144 = t27 * t143;
        let t147 = param_d_4;
        let t148 = t147 * sigma[ip];
        let t151 = param_d_5;
        let t152 = t131 * t131;
        let t154 = t148 * t126 * t131 + 2.0 * t139 * t144 + t151 * t152;
        let t155 = t134 * t121;
        let t156 = 1.0 / t155;
        let t158 = t40 * t115 + t117 / t121 + t133 * t135 + t154 * t156;
        let t162 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t158);
        let tzk0 = 2.0 * t162;
        zk[ip] += tzk0;
        let t164 = t18 / t31;
        let t168 = t37 * t37;
        let t171 = 1.0 / t168 * t21 * t25;
        let t172 = t30 * rho[ip];
        let t174 = 1.0 / t31 / t172;
        let t179 = t42 * tau[ip];
        let t183 = t51 * t59;
        let t184 = t46 * t33;
        let t187 = t55 * t50;
        let t188 = t187 * t59;
        let t191 = t57 * t65;
        let t194 = t61 * t56;
        let t195 = t194 * t65;
        let t198 = t63 * t71;
        let t201 = t67 * t62;
        let t202 = t201 * t71;
        let t205 = t69 * t77;
        let t208 = t73 * t68;
        let t209 = t208 * t77;
        let t212 = t75 * t83;
        let t215 = t79 * t74;
        let t216 = t215 * t83;
        let t219 = 5.0 / 3.0 * t179 * t126 * t53 + 5.0 / 3.0 * t183 * t184 + 10.0 / 3.0 * t188 * t184 + 10.0 / 3.0 * t191 * t184 + 5.0 * t195 * t184 + 5.0 * t198 * t184 + 20.0 / 3.0 * t202 * t184 + 20.0 / 3.0 * t205 * t184 + 25.0 / 3.0 * t209 * t184 + 25.0 / 3.0 * t212 * t184 + 10.0 * t216 * t184;
        let t220 = t81 * t89;
        let t223 = t85 * t80;
        let t224 = t223 * t89;
        let t227 = t87 * t95;
        let t230 = t91 * t86;
        let t231 = t230 * t95;
        let t234 = t93 * t101;
        let t237 = t97 * t92;
        let t238 = t237 * t101;
        let t241 = t99 * t107;
        let t244 = t103 * t98;
        let t245 = t244 * t107;
        let t248 = t105 * t113;
        let t251 = t109 * t104;
        let t252 = t251 * t113;
        let t256 = 1.0 / t94 / t70;
        let t257 = t111 * t256;
        let t260 = 10.0 * t220 * t184 + 35.0 / 3.0 * t224 * t184 + 35.0 / 3.0 * t227 * t184 + 40.0 / 3.0 * t231 * t184 + 40.0 / 3.0 * t234 * t184 + 15.0 * t238 * t184 + 15.0 * t241 * t184 + 50.0 / 3.0 * t245 * t184 + 50.0 / 3.0 * t248 * t184 + 55.0 / 3.0 * t252 * t184 + 55.0 / 3.0 * t257 * t184;
        let t261 = t219 + t260;
        let t263 = t117 * t135;
        let t267 = -0.497936e-2 * t29 * t174 - 0.62242e-2 * t184;
        let t269 = t28 * t174;
        let t272 = t128 * tau[ip];
        let t275 = -8.0 / 3.0 * t125 * t269 - 10.0 / 3.0 * t272 * t126;
        let t277 = t133 * t156;
        let t280 = t140 * t30;
        let t282 = 1.0 / t19 / t280;
        let t283 = t27 * t282;
        let t289 = t144 * tau[ip];
        let t292 = t151 * t131;
        let t295 = -32.0 / 3.0 * t139 * t283 - 8.0 / 3.0 * t148 * t269 * t131 - 20.0 / 3.0 * t148 * t289 - 20.0 / 3.0 * t292 * t184;
        let t297 = t134 * t134;
        let t298 = 1.0 / t297;
        let t299 = t154 * t298;
        let t302 = -0.15766443403838676191e-1 * t171 * t29 * t174 * t115 + t40 * t261 - t263 * t267 + t275 * t135 - 2.0 * t277 * t267 + t295 * t156 - 3.0 * t299 * t267;
        let t307 = piecewise3(t3, 0.0, -t7 * t164 * t158 / 8.0 - 3.0 / 8.0 * t7 * t20 * t302);
        let tvrho0 = 2.0 * rho[ip] * t307 + 2.0 * t162;
        vrho[ip] += tvrho0;
        let t313 = t263 * t126;
        let t315 = t124 * t28;
        let t316 = t33 * t135;
        let t318 = t277 * t126;
        let t320 = t137 * sigma[ip];
        let t323 = t147 * t28;
        let t326 = t323 * t33 * t131 + 4.0 * t320 * t144;
        let t328 = t299 * t126;
        let t330 = 0.59124162764395035716e-2 * t171 * t126 * t115 - 0.186726e-2 * t313 + t315 * t316 - 0.373452e-2 * t318 + t326 * t156 - 0.560178e-2 * t328;
        let t334 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t330);
        let tvsigma0 = 2.0 * rho[ip] * t334;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t336 = t42 * t28;
        let t339 = t59 * t28;
        let t340 = t339 * t48;
        let t344 = t65 * t28;
        let t345 = t344 * t48;
        let t350 = t71 * t28;
        let t351 = t350 * t48;
        let t356 = t77 * t28;
        let t357 = t356 * t48;
        let t362 = t83 * t28;
        let t363 = t362 * t48;
        let t368 = -t336 * t48 * t53 - 2.0 * t187 * t340 - 3.0 * t194 * t345 - 4.0 * t201 * t351 - 5.0 * t208 * t357 - 6.0 * t215 * t363 - t51 * t340 - 2.0 * t57 * t345 - 3.0 * t63 * t351 - 4.0 * t69 * t357 - 5.0 * t75 * t363;
        let t369 = t89 * t28;
        let t370 = t369 * t48;
        let t375 = t95 * t28;
        let t376 = t375 * t48;
        let t381 = t101 * t28;
        let t382 = t381 * t48;
        let t387 = t107 * t28;
        let t388 = t387 * t48;
        let t393 = t113 * t28;
        let t394 = t393 * t48;
        let t399 = t256 * t28;
        let t403 = -11.0 * t111 * t399 * t48 - 10.0 * t105 * t394 - 7.0 * t223 * t370 - 8.0 * t230 * t376 - 9.0 * t237 * t382 - 10.0 * t244 * t388 - 11.0 * t251 * t394 - 6.0 * t81 * t370 - 7.0 * t87 * t376 - 8.0 * t93 * t382 - 9.0 * t99 * t388;
        let t404 = t368 + t403;
        let t406 = t28 * t48;
        let t409 = t128 * t28;
        let t416 = 1.0 / t19 / t140;
        let t417 = t27 * t416;
        let t421 = 4.0 * t148 * t417 + 4.0 * t292 * t406;
        let t425 = t40 * t404 - 0.373452e-2 * t263 * t406 + 2.0 * t409 * t48 * t135 - 0.746904e-2 * t277 * t406 + t421 * t156 - 0.1120356e-1 * t299 * t406;
        let t429 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t425);
        let tvtau0 = 2.0 * rho[ip] * t429;
        vtau[ip] += tvtau0;
    }
}
