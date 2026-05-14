//! MGGA_K_GEA2 fxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_gea2.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_k_gea2_fxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
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
        let t20 = piecewise5(t12, t13, t16, t17, t18 * t9);
        let t21 = 1.0 + t20;
        let t22 = t21 <= zeta_threshold;
        let t23 = pow_1_3(zeta_threshold);
        let t24 = t23 * t23;
        let t25 = t24 * zeta_threshold;
        let t26 = pow_1_3(t21);
        let t27 = t26 * t26;
        let t29 = piecewise3(t22, t25, t27 * t21);
        let t30 = pow_1_3(t8);
        let t31 = t30 * t30;
        let t32 = t29 * t31;
        let t33 = M_CBRT6;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = 1.0 / t36;
        let t38 = t33 * t37;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t48 = 1.0 / t41 / rho0;
        let t52 = 1.0 + 5.0 / 648.0 * t38 * sigma0 * t43 + 5.0 / 54.0 * t38 * lapl0 * t48;
        let t56 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t32 * t52);
        let t57 = rho1 <= dens_threshold;
        let t58 = -t18;
        let t60 = piecewise5(t16, t13, t12, t17, t58 * t9);
        let t61 = 1.0 + t60;
        let t62 = t61 <= zeta_threshold;
        let t63 = pow_1_3(t61);
        let t64 = t63 * t63;
        let t66 = piecewise3(t62, t25, t64 * t61);
        let t67 = t66 * t31;
        let t68 = rho1 * rho1;
        let t69 = pow_1_3(rho1);
        let t70 = t69 * t69;
        let t72 = 1.0 / t70 / t68;
        let t77 = 1.0 / t70 / rho1;
        let t81 = 1.0 + 5.0 / 648.0 * t38 * sigma2 * t72 + 5.0 / 54.0 * t38 * lapl1 * t77;
        let t85 = piecewise3(t57, 0.0, 3.0 / 20.0 * t7 * t67 * t81);
        let tzk0 = t56 + t85;
        zk[ip] += tzk0;
        let t86 = t8 * t8;
        let t87 = 1.0 / t86;
        let t88 = t18 * t87;
        let t90 = piecewise5(t12, 0.0, t16, 0.0, t9 - t88);
        let t93 = piecewise3(t22, 0.0, 5.0 / 3.0 * t27 * t90);
        let t94 = t93 * t31;
        let t98 = 1.0 / t30;
        let t99 = t29 * t98;
        let t102 = t7 * t99 * t52 / 10.0;
        let t105 = 1.0 / t41 / t39 / rho0;
        let t112 = -5.0 / 243.0 * t38 * sigma0 * t105 - 25.0 / 162.0 * t38 * lapl0 * t43;
        let t117 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t94 * t52 + t102 + 3.0 / 20.0 * t7 * t32 * t112);
        let t118 = t58 * t87;
        let t120 = piecewise5(t16, 0.0, t12, 0.0, -t9 - t118);
        let t123 = piecewise3(t62, 0.0, 5.0 / 3.0 * t64 * t120);
        let t124 = t123 * t31;
        let t128 = t66 * t98;
        let t131 = t7 * t128 * t81 / 10.0;
        let t133 = piecewise3(t57, 0.0, 3.0 / 20.0 * t7 * t124 * t81 + t131);
        let tvrho0 = t56 + t85 + t8 * (t117 + t133);
        vrho[ip * 2] += tvrho0;
        let t137 = piecewise5(t12, 0.0, t16, 0.0, -t9 - t88);
        let t140 = piecewise3(t22, 0.0, 5.0 / 3.0 * t27 * t137);
        let t141 = t140 * t31;
        let t146 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t141 * t52 + t102);
        let t148 = piecewise5(t16, 0.0, t12, 0.0, t9 - t118);
        let t151 = piecewise3(t62, 0.0, 5.0 / 3.0 * t64 * t148);
        let t152 = t151 * t31;
        let t158 = 1.0 / t70 / t68 / rho1;
        let t165 = -5.0 / 243.0 * t38 * sigma2 * t158 - 25.0 / 162.0 * t38 * lapl1 * t72;
        let t170 = piecewise3(t57, 0.0, 3.0 / 20.0 * t7 * t152 * t81 + t131 + 3.0 / 20.0 * t7 * t67 * t165);
        let tvrho1 = t56 + t85 + t8 * (t146 + t170);
        vrho[ip * 2 + 1] += tvrho1;
        let t173 = t7 * t29;
        let t174 = t31 * t33;
        let t175 = t37 * t43;
        let t176 = t174 * t175;
        let t177 = t173 * t176;
        let t179 = piecewise3(t2, 0.0, t177 / 864.0);
        let tvsigma0 = t8 * t179;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t180 = t7 * t66;
        let t181 = t37 * t72;
        let t182 = t174 * t181;
        let t183 = t180 * t182;
        let t185 = piecewise3(t57, 0.0, t183 / 864.0);
        let tvsigma2 = t8 * t185;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t186 = t37 * t48;
        let t187 = t174 * t186;
        let t190 = piecewise3(t2, 0.0, t173 * t187 / 72.0);
        let tvlapl0 = t8 * t190;
        vlapl[ip * 2] += tvlapl0;
        let t191 = t37 * t77;
        let t192 = t174 * t191;
        let t195 = piecewise3(t57, 0.0, t180 * t192 / 72.0);
        let tvlapl1 = t8 * t195;
        vlapl[ip * 2 + 1] += tvlapl1;
        let tvtau0 = 0.0;
        vtau[ip * 2] += tvtau0;
        let tvtau1 = 0.0;
        vtau[ip * 2 + 1] += tvtau1;
        let t198 = 1.0 / t26;
        let t199 = t90 * t90;
        let t202 = t86 * t8;
        let t203 = 1.0 / t202;
        let t204 = t18 * t203;
        let t207 = piecewise5(t12, 0.0, t16, 0.0, -2.0 * t87 + 2.0 * t204);
        let t211 = piecewise3(t22, 0.0, 10.0 / 9.0 * t198 * t199 + 5.0 / 3.0 * t27 * t207);
        let t212 = t211 * t31;
        let t216 = t93 * t98;
        let t218 = t7 * t216 * t52;
        let t224 = 1.0 / t30 / t8;
        let t225 = t29 * t224;
        let t228 = t7 * t225 * t52 / 30.0;
        let t230 = t7 * t99 * t112;
        let t232 = t39 * t39;
        let t234 = 1.0 / t41 / t232;
        let t241 = 55.0 / 729.0 * t38 * sigma0 * t234 + 100.0 / 243.0 * t38 * lapl0 * t105;
        let t246 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t212 * t52 + t218 / 5.0 + 3.0 / 10.0 * t7 * t94 * t112 - t228 + t230 / 5.0 + 3.0 / 20.0 * t7 * t32 * t241);
        let t247 = 1.0 / t63;
        let t248 = t120 * t120;
        let t251 = t58 * t203;
        let t254 = piecewise5(t16, 0.0, t12, 0.0, 2.0 * t87 + 2.0 * t251);
        let t258 = piecewise3(t62, 0.0, 10.0 / 9.0 * t247 * t248 + 5.0 / 3.0 * t64 * t254);
        let t259 = t258 * t31;
        let t263 = t123 * t98;
        let t265 = t7 * t263 * t81;
        let t267 = t66 * t224;
        let t270 = t7 * t267 * t81 / 30.0;
        let t272 = piecewise3(t57, 0.0, 3.0 / 20.0 * t7 * t259 * t81 + t265 / 5.0 - t270);
        let tv2rho20 = 2.0 * t117 + 2.0 * t133 + t8 * (t246 + t272);
        v2rho2[ip * 3] += tv2rho20;
        let t275 = t198 * t137;
        let t279 = piecewise5(t12, 0.0, t16, 0.0, 2.0 * t204);
        let t283 = piecewise3(t22, 0.0, 10.0 / 9.0 * t275 * t90 + 5.0 / 3.0 * t27 * t279);
        let t284 = t283 * t31;
        let t288 = t140 * t98;
        let t290 = t7 * t288 * t52;
        let t298 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t284 * t52 + t290 / 10.0 + 3.0 / 20.0 * t7 * t141 * t112 + t218 / 10.0 - t228 + t230 / 10.0);
        let t299 = t247 * t148;
        let t303 = piecewise5(t16, 0.0, t12, 0.0, 2.0 * t251);
        let t307 = piecewise3(t62, 0.0, 10.0 / 9.0 * t299 * t120 + 5.0 / 3.0 * t64 * t303);
        let t308 = t307 * t31;
        let t312 = t151 * t98;
        let t314 = t7 * t312 * t81;
        let t321 = t7 * t128 * t165;
        let t324 = piecewise3(t57, 0.0, 3.0 / 20.0 * t7 * t308 * t81 + t314 / 10.0 + t265 / 10.0 - t270 + 3.0 / 20.0 * t7 * t124 * t165 + t321 / 10.0);
        let tv2rho21 = t117 + t133 + t146 + t170 + t8 * (t298 + t324);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t329 = t137 * t137;
        let t334 = piecewise5(t12, 0.0, t16, 0.0, 2.0 * t87 + 2.0 * t204);
        let t338 = piecewise3(t22, 0.0, 10.0 / 9.0 * t198 * t329 + 5.0 / 3.0 * t27 * t334);
        let t339 = t338 * t31;
        let t345 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t339 * t52 + t290 / 5.0 - t228);
        let t346 = t148 * t148;
        let t351 = piecewise5(t16, 0.0, t12, 0.0, -2.0 * t87 + 2.0 * t251);
        let t355 = piecewise3(t62, 0.0, 10.0 / 9.0 * t247 * t346 + 5.0 / 3.0 * t64 * t351);
        let t356 = t355 * t31;
        let t365 = t68 * t68;
        let t367 = 1.0 / t70 / t365;
        let t374 = 55.0 / 729.0 * t38 * sigma2 * t367 + 100.0 / 243.0 * t38 * lapl1 * t158;
        let t379 = piecewise3(t57, 0.0, 3.0 / 20.0 * t7 * t356 * t81 + t314 / 5.0 + 3.0 / 10.0 * t7 * t152 * t165 - t270 + t321 / 5.0 + 3.0 / 20.0 * t7 * t67 * t374);
        let tv2rho22 = 2.0 * t146 + 2.0 * t170 + t8 * (t345 + t379);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t382 = t7 * t93;
        let t383 = t382 * t176;
        let t385 = t98 * t33;
        let t386 = t385 * t175;
        let t387 = t173 * t386;
        let t388 = t387 / 1296.0;
        let t389 = t37 * t105;
        let t390 = t174 * t389;
        let t391 = t173 * t390;
        let t394 = piecewise3(t2, 0.0, t383 / 864.0 + t388 - t391 / 324.0);
        let tv2rhosigma0 = t394 * t8 + t179;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let t396 = t7 * t123;
        let t397 = t396 * t182;
        let t399 = t385 * t181;
        let t400 = t180 * t399;
        let t401 = t400 / 1296.0;
        let t403 = piecewise3(t57, 0.0, t397 / 864.0 + t401);
        let tv2rhosigma2 = t403 * t8 + t185;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let t405 = t7 * t140;
        let t406 = t405 * t176;
        let t409 = piecewise3(t2, 0.0, t406 / 864.0 + t388);
        let tv2rhosigma3 = t409 * t8 + t179;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t411 = t7 * t151;
        let t412 = t411 * t182;
        let t414 = t37 * t158;
        let t415 = t174 * t414;
        let t416 = t180 * t415;
        let t419 = piecewise3(t57, 0.0, t412 / 864.0 + t401 - t416 / 324.0);
        let tv2rhosigma5 = t419 * t8 + t185;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t423 = t385 * t186;
        let t425 = t173 * t423 / 108.0;
        let t428 = piecewise3(t2, 0.0, t382 * t187 / 72.0 + t425 - 5.0 / 216.0 * t177);
        let tv2rholapl0 = t428 * t8 + t190;
        v2rholapl[ip * 4] += tv2rholapl0;
        let t432 = t385 * t191;
        let t434 = t180 * t432 / 108.0;
        let t436 = piecewise3(t57, 0.0, t396 * t192 / 72.0 + t434);
        let tv2rholapl1 = t436 * t8 + t195;
        v2rholapl[ip * 4 + 1] += tv2rholapl1;
        let t441 = piecewise3(t2, 0.0, t405 * t187 / 72.0 + t425);
        let tv2rholapl2 = t441 * t8 + t190;
        v2rholapl[ip * 4 + 2] += tv2rholapl2;
        let t447 = piecewise3(t57, 0.0, t411 * t192 / 72.0 + t434 - 5.0 / 216.0 * t183);
        let tv2rholapl3 = t447 * t8 + t195;
        v2rholapl[ip * 4 + 3] += tv2rholapl3;
        let tv2rhotau0 = 0.0;
        v2rhotau[ip * 4] += tv2rhotau0;
        let tv2rhotau1 = 0.0;
        v2rhotau[ip * 4 + 1] += tv2rhotau1;
        let tv2rhotau2 = 0.0;
        v2rhotau[ip * 4 + 2] += tv2rhotau2;
        let tv2rhotau3 = 0.0;
        v2rhotau[ip * 4 + 3] += tv2rhotau3;
        let tv2sigma20 = 0.0;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = 0.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = 0.0;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = 0.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = 0.0;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let tv2sigma25 = 0.0;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip * 6] += tv2sigmalapl0;
        let tv2sigmalapl1 = 0.0;
        v2sigmalapl[ip * 6 + 1] += tv2sigmalapl1;
        let tv2sigmalapl2 = 0.0;
        v2sigmalapl[ip * 6 + 2] += tv2sigmalapl2;
        let tv2sigmalapl3 = 0.0;
        v2sigmalapl[ip * 6 + 3] += tv2sigmalapl3;
        let tv2sigmalapl4 = 0.0;
        v2sigmalapl[ip * 6 + 4] += tv2sigmalapl4;
        let tv2sigmalapl5 = 0.0;
        v2sigmalapl[ip * 6 + 5] += tv2sigmalapl5;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip * 6] += tv2sigmatau0;
        let tv2sigmatau1 = 0.0;
        v2sigmatau[ip * 6 + 1] += tv2sigmatau1;
        let tv2sigmatau2 = 0.0;
        v2sigmatau[ip * 6 + 2] += tv2sigmatau2;
        let tv2sigmatau3 = 0.0;
        v2sigmatau[ip * 6 + 3] += tv2sigmatau3;
        let tv2sigmatau4 = 0.0;
        v2sigmatau[ip * 6 + 4] += tv2sigmatau4;
        let tv2sigmatau5 = 0.0;
        v2sigmatau[ip * 6 + 5] += tv2sigmatau5;
        let tv2lapl20 = 0.0;
        v2lapl2[ip * 3] += tv2lapl20;
        let tv2lapl21 = 0.0;
        v2lapl2[ip * 3 + 1] += tv2lapl21;
        let tv2lapl22 = 0.0;
        v2lapl2[ip * 3 + 2] += tv2lapl22;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip * 4] += tv2lapltau0;
        let tv2lapltau1 = 0.0;
        v2lapltau[ip * 4 + 1] += tv2lapltau1;
        let tv2lapltau2 = 0.0;
        v2lapltau[ip * 4 + 2] += tv2lapltau2;
        let tv2lapltau3 = 0.0;
        v2lapltau[ip * 4 + 3] += tv2lapltau3;
        let tv2tau20 = 0.0;
        v2tau2[ip * 3] += tv2tau20;
        let tv2tau21 = 0.0;
        v2tau2[ip * 3 + 1] += tv2tau21;
        let tv2tau22 = 0.0;
        v2tau2[ip * 3 + 2] += tv2tau22;
    }
}
