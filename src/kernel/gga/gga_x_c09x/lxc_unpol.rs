//! GGA_X_C09X lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_c09x.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_c09x_lxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = t25 * sigma[ip];
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t33 = t28 * t32;
        let t34 = sigma[ip] * t28;
        let t36 = t25 * t34 * t32;
        let t38 = f64::exp(-0.20125e-2 * t36);
        let t39 = t33 * t38;
        let t43 = f64::exp(-0.100625e-2 * t36);
        let t45 = 0.2245e1 + 0.25708333333333333333e-2 * t26 * t39 - 0.1245e1 * t43;
        let t49 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t45);
        let tzk0 = 2.0 * t49;
        zk[ip] += tzk0;
        let t51 = t17 / t30;
        let t55 = t29 * rho[ip];
        let t57 = 1.0 / t30 / t55;
        let t58 = t28 * t57;
        let t59 = t58 * t38;
        let t62 = t20 * t20;
        let t64 = 1.0 / t22 / t21;
        let t65 = t62 * t64;
        let t66 = sigma[ip] * sigma[ip];
        let t67 = t65 * t66;
        let t68 = t29 * t29;
        let t69 = t68 * t29;
        let t71 = 1.0 / t18 / t69;
        let t72 = t27 * t71;
        let t73 = t72 * t38;
        let t76 = t58 * t43;
        let t79 = -0.68555555555555555555e-2 * t26 * t59 + 0.2759361111111111111e-4 * t67 * t73 - 0.334075e-2 * t26 * t76;
        let t84 = piecewise3(t2, 0.0, -t6 * t51 * t45 / 8.0 - 3.0 / 8.0 * t6 * t19 * t79);
        let tvrho0 = 2.0 * rho[ip] * t84 + 2.0 * t49;
        vrho[ip] += tvrho0;
        let t90 = t68 * rho[ip];
        let t93 = t27 / t18 / t90;
        let t94 = t93 * t38;
        let t100 = 0.25708333333333333333e-2 * t25 * t39 - 0.10347604166666666667e-4 * t65 * sigma[ip] * t94 + 0.125278125e-2 * t25 * t33 * t43;
        let t104 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t100);
        let tvsigma0 = 2.0 * rho[ip] * t104;
        vsigma[ip] += tvsigma0;
        let t109 = t17 / t30 / rho[ip];
        let t118 = t28 / t30 / t68;
        let t119 = t118 * t38;
        let t122 = t68 * t55;
        let t124 = 1.0 / t18 / t122;
        let t125 = t27 * t124;
        let t126 = t125 * t38;
        let t129 = t66 * sigma[ip];
        let t130 = t68 * t68;
        let t131 = t130 * t29;
        let t132 = 1.0 / t131;
        let t136 = t118 * t43;
        let t139 = t125 * t43;
        let t142 = 0.25137037037037037037e-1 * t26 * t119 - 0.24834249999999999999e-3 * t67 * t126 + 0.18242943617400669616e-7 * t129 * t132 * t38 + 0.12249416666666666667e-1 * t26 * t136 - 0.17928691666666666666e-4 * t67 * t139;
        let t147 = piecewise3(t2, 0.0, t6 * t109 * t45 / 12.0 - t6 * t51 * t79 / 4.0 - 3.0 / 8.0 * t6 * t19 * t142);
        let tv2rho20 = 2.0 * rho[ip] * t147 + 4.0 * t84;
        v2rho2[ip] += tv2rho20;
        let t155 = t65 * t27;
        let t156 = t71 * sigma[ip];
        let t160 = t130 * rho[ip];
        let t161 = 1.0 / t160;
        let t170 = -0.68555555555555555555e-2 * t25 * t59 + 0.82780833333333333334e-4 * t155 * t156 * t38 - 0.68411038565252511063e-8 * t66 * t161 * t38 - 0.334075e-2 * t25 * t76 + 0.67232593749999999999e-5 * t155 * t156 * t43;
        let t175 = piecewise3(t2, 0.0, -t6 * t51 * t100 / 8.0 - 3.0 / 8.0 * t6 * t19 * t170);
        let tv2rhosigma0 = 2.0 * rho[ip] * t175 + 2.0 * t104;
        v2rhosigma[ip] += tv2rhosigma0;
        let t180 = 1.0 / t130;
        let t187 = -0.20695208333333333334e-4 * t65 * t94 + 0.25654139461969691649e-8 * sigma[ip] * t180 * t38 - 0.2521222265625e-5 * t65 * t93 * t43;
        let t191 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t187);
        let tv2sigma20 = 2.0 * rho[ip] * t191;
        v2sigma2[ip] += tv2sigma20;
        let t194 = t17 * t32;
        let t206 = t28 / t30 / t90;
        let t207 = t206 * t38;
        let t211 = 1.0 / t18 / t130;
        let t212 = t27 * t211;
        let t216 = t130 * t55;
        let t217 = 1.0 / t216;
        let t218 = t129 * t217;
        let t221 = t66 * t66;
        let t222 = t130 * t90;
        let t224 = 1.0 / t30 / t222;
        let t227 = t24 * t28;
        let t228 = t227 * t38;
        let t231 = t206 * t43;
        let t239 = -0.11730617283950617284e0 * t26 * t207 + 0.20909825308641975308e-2 * t67 * t212 * t38 - 0.3466159287306127227e-6 * t218 * t38 + 0.97903797413383593606e-10 * t221 * t224 * t20 * t228 - 0.57163944444444444446e-1 * t26 * t231 + 0.19721560833333333333e-3 * t67 * t212 * t43 - 0.59265913020905627571e-8 * t218 * t43;
        let t244 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t194 * t45 + t6 * t109 * t79 / 4.0 - 3.0 / 8.0 * t6 * t51 * t142 - 3.0 / 8.0 * t6 * t19 * t239);
        let tv3rho30 = 2.0 * rho[ip] * t244 + 6.0 * t147;
        v3rho3[ip] += tv3rho30;
        let t256 = t124 * sigma[ip];
        let t260 = t132 * t66;
        let t263 = t130 * t68;
        let t265 = 1.0 / t30 / t263;
        let t277 = 0.25137037037037037037e-1 * t25 * t119 - 0.59786157407407407408e-3 * t155 * t256 * t38 + 0.11629876556092926881e-6 * t260 * t38 - 0.36713924030018847604e-10 * t129 * t265 * t20 * t228 + 0.12249416666666666667e-1 * t25 * t136 - 0.60509334374999999999e-4 * t155 * t256 * t43 + 0.2222471738283961034e-8 * t260 * t43;
        let t282 = piecewise3(t2, 0.0, t6 * t109 * t100 / 12.0 - t6 * t51 * t170 / 4.0 - 3.0 / 8.0 * t6 * t19 * t277);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t282 + 4.0 * t175;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t290 = t161 * sigma[ip];
        let t294 = 1.0 / t30 / t216;
        let t304 = 0.11037444444444444445e-3 * t65 * t73 - 0.34205519282626255532e-7 * t290 * t38 + 0.13767721511257067852e-10 * t66 * t294 * t20 * t228 + 0.1344651875e-4 * t65 * t72 * t43 - 0.83342690185648538775e-9 * t290 * t43;
        let t309 = piecewise3(t2, 0.0, -t6 * t51 * t187 / 8.0 - 3.0 / 8.0 * t6 * t19 * t304);
        let tv3rhosigma20 = 2.0 * rho[ip] * t309 + 2.0 * t191;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t315 = 1.0 / t30 / t131;
        let t322 = 0.76962418385909074947e-8 * t180 * t38 - 0.51628955667214004444e-11 * sigma[ip] * t315 * t20 * t228 + 0.31253508819618202039e-9 * t180 * t43;
        let t326 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t322);
        let tv3sigma30 = 2.0 * rho[ip] * t326;
        v3sigma3[ip] += tv3sigma30;
        let t344 = t28 / t30 / t69;
        let t350 = t27 / t18 / t160;
        let t355 = t129 / t263;
        let t358 = t130 * t69;
        let t362 = t221 / t30 / t358 * t20;
        let t366 = t130 * t130;
        let t373 = t64 * t27 * t38;
        let t384 = t227 * t43;
        let t392 = piecewise3(t2, 0.0, 10.0 / 27.0 * t6 * t17 * t57 * t45 - 5.0 / 9.0 * t6 * t194 * t79 + t6 * t109 * t142 / 2.0 - t6 * t51 * t239 / 2.0 - 3.0 / 8.0 * t6 * t19 * (0.66473497942386831276e0 * t26 * t344 * t38 - 0.18683940679012345679e-1 * t67 * t350 * t38 + 0.51951849434886573584e-5 * t355 * t38 - 0.31981907155038640578e-8 * t362 * t228 + 0.10508340922369839047e-11 * t221 * sigma[ip] / t18 / t366 / rho[ip] * t62 * t373 + 0.32392901851851851853e0 * t26 * t344 * t43 - 0.19502432379629629629e-2 * t67 * t350 * t43 + 0.13038500864599238066e-6 * t355 * t43 - 0.15903019993943010065e-10 * t362 * t384));
        let tv4rho40 = 2.0 * rho[ip] * t392 + 8.0 * t244;
        v4rho4[ip] += tv4rho40;
        let t407 = t211 * sigma[ip];
        let t411 = t217 * t66;
        let t415 = t224 * t129 * t20;
        let t438 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t194 * t100 + t6 * t109 * t170 / 4.0 - 3.0 / 8.0 * t6 * t51 * t277 - 3.0 / 8.0 * t6 * t19 * (-0.11730617283950617284e0 * t25 * t207 + 0.46541224074074074074e-2 * t155 * t407 * t38 - 0.15582514339863071965e-5 * t411 * t38 + 0.10891797462238924789e-8 * t415 * t228 - 0.39406278458886896427e-12 * t221 / t18 / t366 * t62 * t373 - 0.57163944444444444446e-1 * t25 * t231 + 0.5094736548611111111e-3 * t155 * t407 * t43 - 0.42226963027395259646e-7 * t411 * t43 + 0.59636324977286287745e-11 * t415 * t384));
        let tv4rho3sigma0 = 2.0 * rho[ip] * t438 + 6.0 * t282;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t450 = t132 * sigma[ip];
        let t454 = t265 * t66 * t20;
        let t475 = piecewise3(t2, 0.0, t6 * t109 * t187 / 12.0 - t6 * t51 * t304 / 4.0 - 3.0 / 8.0 * t6 * t19 * (-0.69903814814814814818e-3 * t65 * t126 + 0.38082144801323897826e-6 * t450 * t38 - 0.34419303778142669629e-9 * t454 * t228 + 0.14777354422082586161e-12 * t129 / t18 / t130 / t122 * t62 * t373 - 0.85161285416666666667e-4 * t65 * t139 + 0.11945785593276290558e-7 * t450 * t43 - 0.22363621866482357904e-11 * t454 * t384));
        let tv4rho2sigma20 = 2.0 * rho[ip] * t475 + 4.0 * t309;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t484 = t294 * t20 * t24;
        let t504 = piecewise3(t2, 0.0, -t6 * t51 * t322 / 8.0 - 3.0 / 8.0 * t6 * t19 * (-0.61569934708727259958e-7 * t161 * t38 + 0.96374050578799474962e-10 * t484 * t34 * t38 - 0.55415079082809698102e-13 * t66 / t18 / t358 * t62 * t373 - 0.25002807055694561631e-8 * t161 * t43 + 0.83863581999308842137e-12 * t484 * t34 * t43));
        let tv4rhosigma30 = 2.0 * rho[ip] * t504 + 2.0 * t326;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t507 = t315 * t20;
        let t522 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * (-0.20651582266885601777e-10 * t507 * t228 + 0.20780654656053636789e-13 * sigma[ip] / t18 / t222 * t62 * t373 - 0.31448843249740815802e-12 * t507 * t384));
        let tv4sigma40 = 2.0 * rho[ip] * t522;
        v4sigma4[ip] += tv4sigma40;
    }
}
