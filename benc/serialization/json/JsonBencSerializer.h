/* vim: set expandtab ts=4 sw=4: */
/*
 * You may redistribute this program and/or modify it under the terms of
 * the GNU General Public License as published by the Free Software Foundation,
 * either version 3 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
#ifndef JsonBencSerializer_H
#define JsonBencSerializer_H

#include "benc/serialization/BencSerializer.h"
#include "util/Linker.h"
Linker_require("benc/serialization/json/JsonBencSerializer.c");

const struct BencSerializer* JsonBencSerializer_get(void);
#endif
