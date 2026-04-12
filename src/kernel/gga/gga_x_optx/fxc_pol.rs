//! GGA_X_OPTX fxc pol kernel.
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
pub fn gga_x_optx_fxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
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
        let t218 = t23 * t23;
        let t219 = 1.0 / t218;
        let t220 = t90 * t90;
        let t223 = t86 * t6;
        let t224 = 1.0 / t223;
        let t225 = t16 * t224;
        let t228 = piecewise5(t10, 0.0, t14, 0.0, -2.0 * t87 + 2.0 * t225);
        let t232 = piecewise3(t20, 0.0, 4.0 / 9.0 * t219 * t220 + 4.0 / 3.0 * t23 * t228);
        let t233 = t232 * t26;
        let t237 = t93 * t99;
        let t239 = t5 * t237 * t48;
        let t245 = 1.0 / t98 / t6;
        let t246 = t25 * t245;
        let t249 = t5 * t246 * t48 / 12.0;
        let t251 = t5 * t100 * t122;
        let t253 = t31 * rho0;
        let t254 = t32 * t253;
        let t256 = 1.0 / t34 / t254;
        let t261 = t113 * t31;
        let t262 = 1.0 / t261;
        let t267 = t28 * t28;
        let t268 = param_b * t267;
        let t269 = t30 * t30;
        let t270 = t113 * t32;
        let t272 = 1.0 / t39 / t270;
        let t274 = t44 * t44;
        let t275 = 1.0 / t274;
        let t279 = 304.0 / 9.0 * t29 * t30 * t256 * t45 - 688.0 / 9.0 * t111 * t112 * t262 * t118 + 128.0 / 3.0 * t268 * t269 * t272 * t275;
        let t284 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t233 * t48 - t239 / 4.0 - 3.0 / 4.0 * t5 * t94 * t122 + t249 - t251 / 4.0 - 3.0 / 8.0 * t5 * t27 * t279);
        let t285 = t59 * t59;
        let t286 = 1.0 / t285;
        let t287 = t130 * t130;
        let t290 = t54 * t224;
        let t293 = piecewise5(t14, 0.0, t10, 0.0, 2.0 * t87 + 2.0 * t290);
        let t297 = piecewise3(t58, 0.0, 4.0 / 9.0 * t286 * t287 + 4.0 / 3.0 * t59 * t293);
        let t298 = t297 * t26;
        let t302 = t133 * t99;
        let t304 = t5 * t302 * t81;
        let t306 = t61 * t245;
        let t309 = t5 * t306 * t81 / 12.0;
        let t311 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t298 * t81 - t304 / 4.0 + t309);
        let tv2rho20 = 2.0 * t127 + 2.0 * t143 + t6 * (t284 + t311);
        v2rho2[ip * 3] += tv2rho20;
        let t314 = t219 * t147;
        let t318 = piecewise5(t10, 0.0, t14, 0.0, 2.0 * t225);
        let t322 = piecewise3(t20, 0.0, 4.0 / 9.0 * t314 * t90 + 4.0 / 3.0 * t23 * t318);
        let t323 = t322 * t26;
        let t327 = t150 * t99;
        let t329 = t5 * t327 * t48;
        let t337 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t323 * t48 - t329 / 8.0 - 3.0 / 8.0 * t5 * t151 * t122 - t239 / 8.0 + t249 - t251 / 8.0);
        let t338 = t286 * t158;
        let t342 = piecewise5(t14, 0.0, t10, 0.0, 2.0 * t290);
        let t346 = piecewise3(t58, 0.0, 4.0 / 9.0 * t338 * t130 + 4.0 / 3.0 * t59 * t342);
        let t347 = t346 * t26;
        let t351 = t161 * t99;
        let t353 = t5 * t351 * t81;
        let t360 = t5 * t138 * t182;
        let t363 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t347 * t81 - t353 / 8.0 - t304 / 8.0 + t309 - 3.0 / 8.0 * t5 * t134 * t182 - t360 / 8.0);
        let tv2rho21 = t127 + t143 + t156 + t187 + t6 * (t337 + t363);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t368 = t147 * t147;
        let t373 = piecewise5(t10, 0.0, t14, 0.0, 2.0 * t87 + 2.0 * t225);
        let t377 = piecewise3(t20, 0.0, 4.0 / 9.0 * t219 * t368 + 4.0 / 3.0 * t23 * t373);
        let t378 = t377 * t26;
        let t384 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t378 * t48 - t329 / 4.0 + t249);
        let t385 = t158 * t158;
        let t390 = piecewise5(t14, 0.0, t10, 0.0, -2.0 * t87 + 2.0 * t290);
        let t394 = piecewise3(t58, 0.0, 4.0 / 9.0 * t286 * t385 + 4.0 / 3.0 * t59 * t390);
        let t395 = t394 * t26;
        let t404 = t64 * rho1;
        let t405 = t65 * t404;
        let t407 = 1.0 / t67 / t405;
        let t412 = t173 * t64;
        let t413 = 1.0 / t412;
        let t418 = t63 * t63;
        let t419 = t173 * t65;
        let t421 = 1.0 / t72 / t419;
        let t423 = t77 * t77;
        let t424 = 1.0 / t423;
        let t428 = 304.0 / 9.0 * t29 * t63 * t407 * t78 - 688.0 / 9.0 * t111 * t172 * t413 * t178 + 128.0 / 3.0 * t268 * t418 * t421 * t424;
        let t433 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t395 * t81 - t353 / 4.0 - 3.0 / 4.0 * t5 * t162 * t182 + t309 - t360 / 4.0 - 3.0 / 8.0 * t5 * t62 * t428);
        let tv2rho22 = 2.0 * t156 + 2.0 * t187 + t6 * (t384 + t433);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t441 = t5 * t100 * t198 / 8.0;
        let t450 = t113 * t253;
        let t452 = 1.0 / t39 / t450;
        let t457 = -32.0 / 3.0 * t29 * sigma0 * t106 * t45 + 80.0 / 3.0 * t111 * t30 * t115 * t118 - 16.0 * t268 * t112 * t452 * t275;
        let t462 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t94 * t198 - t441 - 3.0 / 8.0 * t5 * t27 * t457);
        let tv2rhosigma0 = t6 * t462 + t202;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let t469 = t5 * t138 * t211 / 8.0;
        let t471 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t134 * t211 - t469);
        let tv2rhosigma2 = t6 * t471 + t215;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let t477 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t151 * t198 - t441);
        let tv2rhosigma3 = t6 * t477 + t202;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t490 = t173 * t404;
        let t492 = 1.0 / t72 / t490;
        let t497 = -32.0 / 3.0 * t29 * sigma2 * t168 * t78 + 80.0 / 3.0 * t111 * t63 * t175 * t178 - 16.0 * t268 * t172 * t492 * t424;
        let t502 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t162 * t211 - t469 - 3.0 / 8.0 * t5 * t62 * t497);
        let tv2rhosigma5 = t6 * t502 + t215;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t512 = 1.0 / t39 / t261;
        let t517 = -8.0 * t111 * sigma0 * t193 * t118 + 6.0 * t268 * t30 * t512 * t275 + 2.0 * t29 * t36 * t45;
        let t521 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t517);
        let tv2sigma20 = t6 * t521;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = 0.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = 0.0;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = 0.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = 0.0;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let t530 = 1.0 / t72 / t412;
        let t535 = -8.0 * t111 * sigma2 * t206 * t178 + 6.0 * t268 * t63 * t530 * t424 + 2.0 * t29 * t69 * t78;
        let t539 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t62 * t535);
        let tv2sigma25 = t6 * t539;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
    }
}
