//! GGA_X_2D_B88 kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_2d_b88.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_PI, M_SQRT2};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_2d_b88_kxc_unpol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = f64::sqrt(M_PI);
        let t5 = 1.0 <= zeta_threshold;
        let t6 = zeta_threshold - 1.0;
        let t8 = piecewise5(t5, t6, t5, -t6, 0.0);
        let t9 = 1.0 + t8;
        let t11 = f64::sqrt(zeta_threshold);
        let t13 = f64::sqrt(t9);
        let t15 = piecewise3(t9 <= zeta_threshold, t11 * zeta_threshold, t13 * t9);
        let t16 = 1.0 / t3 * t15;
        let t17 = M_SQRT2;
        let t18 = f64::sqrt(rho[ip]);
        let t19 = t17 * t18;
        let t20 = rho[ip] * rho[ip];
        let t21 = t20 * rho[ip];
        let t22 = 1.0 / t21;
        let t23 = sigma[ip] * t22;
        let t24 = f64::sqrt(sigma[ip]);
        let t25 = t24 * t17;
        let t27 = 1.0 / t18 / rho[ip];
        let t29 = f64::ln(t25 * t27 + f64::sqrt(pow_2(t25 * t27) + 1.0));
        let t30 = t27 * t29;
        let t33 = 1.0 + 0.56e-1 * t25 * t30;
        let t34 = 1.0 / t33;
        let t37 = 1.0 + 0.93053827172539591434e-2 * t23 * t34;
        let t41 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t37);
        let tzk0 = 2.0 * t41;
        zk[ip] += tzk0;
        let t43 = t17 / t18;
        let t47 = t20 * t20;
        let t48 = 1.0 / t47;
        let t49 = sigma[ip] * t48;
        let t52 = t33 * t33;
        let t53 = 1.0 / t52;
        let t55 = 1.0 / t18 / t20;
        let t56 = t55 * t29;
        let t60 = 2.0 * t23 + 1.0;
        let t61 = f64::sqrt(t60);
        let t62 = 1.0 / t61;
        let t65 = -0.84e-1 * t25 * t56 - 0.168e0 * t49 * t62;
        let t66 = t53 * t65;
        let t69 = -0.2791614815176187743e-1 * t49 * t34 - 0.93053827172539591434e-2 * t23 * t66;
        let t74 = piecewise3(t2, 0.0, -t16 * t43 * t37 / 3.0 - 2.0 / 3.0 * t16 * t19 * t69);
        let tvrho0 = 2.0 * rho[ip] * t74 + 2.0 * t41;
        vrho[ip] += tvrho0;
        let t80 = 1.0 / t24 * t17;
        let t85 = 0.28e-1 * t80 * t30 + 0.56e-1 * t22 * t62;
        let t86 = t53 * t85;
        let t89 = 0.93053827172539591434e-2 * t22 * t34 - 0.93053827172539591434e-2 * t23 * t86;
        let t93 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t89);
        let tvsigma0 = 2.0 * rho[ip] * t93;
        vsigma[ip] += tvsigma0;
        let t96 = t17 * t27;
        let t103 = t47 * rho[ip];
        let t104 = 1.0 / t103;
        let t105 = sigma[ip] * t104;
        let t111 = 1.0 / t52 / t33;
        let t112 = t65 * t65;
        let t113 = t111 * t112;
        let t117 = 1.0 / t18 / t21;
        let t118 = t117 * t29;
        let t123 = sigma[ip] * sigma[ip];
        let t124 = t47 * t47;
        let t125 = 1.0 / t124;
        let t128 = 1.0 / t61 / t60;
        let t131 = 0.21e0 * t25 * t118 + 0.924e0 * t105 * t62 - 0.504e0 * t123 * t125 * t128;
        let t132 = t53 * t131;
        let t135 = 0.11166459260704750972e0 * t105 * t34 + 0.5583229630352375486e-1 * t49 * t66 + 0.18610765434507918287e-1 * t23 * t113 - 0.93053827172539591434e-2 * t23 * t132;
        let t140 = piecewise3(t2, 0.0, t16 * t96 * t37 / 6.0 - 2.0 / 3.0 * t16 * t43 * t69 - 2.0 / 3.0 * t16 * t19 * t135);
        let tv2rho20 = 2.0 * rho[ip] * t140 + 4.0 * t74;
        v2rho2[ip] += tv2rho20;
        let t148 = t22 * t53;
        let t153 = t111 * t85;
        let t154 = t153 * t65;
        let t161 = t47 * t21;
        let t162 = 1.0 / t161;
        let t163 = t162 * t128;
        let t166 = -0.42e-1 * t80 * t56 - 0.252e0 * t48 * t62 + 0.168e0 * t163 * sigma[ip];
        let t167 = t53 * t166;
        let t170 = -0.2791614815176187743e-1 * t48 * t34 - 0.93053827172539591434e-2 * t148 * t65 + 0.2791614815176187743e-1 * t49 * t86 + 0.18610765434507918287e-1 * t23 * t154 - 0.93053827172539591434e-2 * t23 * t167;
        let t175 = piecewise3(t2, 0.0, -t16 * t43 * t89 / 3.0 - 2.0 / 3.0 * t16 * t19 * t170);
        let tv2rhosigma0 = 2.0 * rho[ip] * t175 + 2.0 * t93;
        v2rhosigma[ip] += tv2rhosigma0;
        let t180 = t85 * t85;
        let t181 = t111 * t180;
        let t186 = 1.0 / t24 / sigma[ip] * t17;
        let t189 = 1.0 / sigma[ip];
        let t193 = t47 * t20;
        let t194 = 1.0 / t193;
        let t197 = -0.14e-1 * t186 * t30 + 0.28e-1 * t189 * t22 * t62 - 0.56e-1 * t194 * t128;
        let t198 = t53 * t197;
        let t201 = -0.18610765434507918287e-1 * t148 * t85 + 0.18610765434507918287e-1 * t23 * t181 - 0.93053827172539591434e-2 * t23 * t198;
        let t205 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t201);
        let tv2sigma20 = 2.0 * rho[ip] * t205;
        v2sigma2[ip] += tv2sigma20;
        let t208 = t17 * t55;
        let t217 = sigma[ip] * t194;
        let t226 = t52 * t52;
        let t227 = 1.0 / t226;
        let t228 = t112 * t65;
        let t229 = t227 * t228;
        let t232 = t111 * t65;
        let t233 = t232 * t131;
        let t238 = 1.0 / t18 / t47 * t29;
        let t244 = 1.0 / t124 / rho[ip];
        let t248 = t123 * sigma[ip];
        let t250 = 1.0 / t124 / t47;
        let t252 = t60 * t60;
        let t254 = 1.0 / t61 / t252;
        let t257 = -0.735e0 * t25 * t238 - 0.525e1 * t217 * t62 + 0.6804e1 * t123 * t244 * t128 - 0.4536e1 * t248 * t250 * t254;
        let t258 = t53 * t257;
        let t261 = -0.5583229630352375486e0 * t217 * t34 - 0.33499377782114252916e0 * t105 * t66 - 0.16749688891057126458e0 * t49 * t113 + 0.8374844445528563229e-1 * t49 * t132 - 0.55832296303523754861e-1 * t23 * t229 + 0.55832296303523754861e-1 * t23 * t233 - 0.93053827172539591434e-2 * t23 * t258;
        let t266 = piecewise3(t2, 0.0, -t16 * t208 * t37 / 4.0 + t16 * t96 * t69 / 2.0 - t16 * t43 * t135 - 2.0 / 3.0 * t16 * t19 * t261);
        let tv3rho30 = 2.0 * rho[ip] * t266 + 6.0 * t140;
        v3rho3[ip] += tv3rho30;
        let t278 = t48 * t53;
        let t281 = t22 * t111;
        let t293 = t227 * t85 * t112;
        let t296 = t111 * t166;
        let t297 = t296 * t65;
        let t300 = t153 * t131;
        let t307 = t125 * t128;
        let t312 = 1.0 / t124 / t21 * t254;
        let t315 = 0.105e0 * t80 * t118 + 0.1134e1 * t104 * t62 - 0.1932e1 * t307 * sigma[ip] + 0.1512e1 * t312 * t123;
        let t316 = t53 * t315;
        let t319 = 0.11166459260704750972e0 * t104 * t34 + 0.5583229630352375486e-1 * t278 * t65 + 0.18610765434507918287e-1 * t281 * t112 - 0.93053827172539591434e-2 * t148 * t131 - 0.11166459260704750972e0 * t105 * t86 - 0.11166459260704750972e0 * t49 * t154 + 0.5583229630352375486e-1 * t49 * t167 - 0.55832296303523754861e-1 * t23 * t293 + 0.37221530869015836574e-1 * t23 * t297 + 0.18610765434507918287e-1 * t23 * t300 - 0.93053827172539591434e-2 * t23 * t316;
        let t324 = piecewise3(t2, 0.0, t16 * t96 * t89 / 6.0 - 2.0 / 3.0 * t16 * t43 * t170 - 2.0 / 3.0 * t16 * t19 * t319);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t324 + 4.0 * t175;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t332 = t85 * t65;
        let t339 = t227 * t180;
        let t340 = t339 * t65;
        let t343 = t153 * t166;
        let t348 = t111 * t197;
        let t349 = t348 * t65;
        let t359 = 1.0 / t124 / t20;
        let t360 = t359 * t254;
        let t363 = 0.21e-1 * t186 * t56 - 0.42e-1 * t189 * t48 * t62 + 0.42e0 * t163 - 0.504e0 * t360 * sigma[ip];
        let t364 = t53 * t363;
        let t367 = 0.55832296303523754861e-1 * t278 * t85 + 0.37221530869015836574e-1 * t281 * t332 - 0.18610765434507918287e-1 * t148 * t166 - 0.55832296303523754861e-1 * t49 * t181 - 0.55832296303523754861e-1 * t23 * t340 + 0.37221530869015836574e-1 * t23 * t343 + 0.2791614815176187743e-1 * t49 * t198 + 0.18610765434507918287e-1 * t23 * t349 - 0.93053827172539591434e-2 * t23 * t364;
        let t372 = piecewise3(t2, 0.0, -t16 * t43 * t201 / 3.0 - 2.0 / 3.0 * t16 * t19 * t367);
        let tv3rhosigma20 = 2.0 * rho[ip] * t372 + 2.0 * t205;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t379 = t180 * t85;
        let t380 = t227 * t379;
        let t383 = t153 * t197;
        let t388 = 1.0 / t24 / t123 * t17;
        let t391 = 1.0 / t123;
        let t400 = 0.21e-1 * t388 * t30 - 0.42e-1 * t391 * t22 * t62 - 0.28e-1 * t189 * t194 * t128 + 0.168e0 * t244 * t254;
        let t401 = t53 * t400;
        let t404 = 0.55832296303523754861e-1 * t281 * t180 - 0.2791614815176187743e-1 * t148 * t197 - 0.55832296303523754861e-1 * t23 * t380 + 0.55832296303523754861e-1 * t23 * t383 - 0.93053827172539591434e-2 * t23 * t401;
        let t408 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t404);
        let tv3sigma30 = 2.0 * rho[ip] * t408;
        v3sigma3[ip] += tv3sigma30;
    }
}
