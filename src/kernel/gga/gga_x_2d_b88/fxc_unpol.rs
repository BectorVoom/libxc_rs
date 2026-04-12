//! GGA_X_2D_B88 fxc unpol kernel.
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
pub fn gga_x_2d_b88_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
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
    }
}
