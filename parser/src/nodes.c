#include "nodes.h"
#include "C.tab.h"
#include <stdio.h>
#include <stdlib.h>

NODE *make_node(int t, NODE *left, NODE *right) {
  NODE *a = (NODE *)malloc(sizeof(NODE));
  if (a == NULL) {
    perror("Cannot make node");
    exit(1);
  }
  a->type = t;
  a->left = left;
  a->right = right;
  return a;
}

NODE *make_leaf(TOKEN *l) {
  NODE *a = (NODE *)malloc(sizeof(NODE));
  /*     printf("make_leaf: %p -> %p\n", l, a); */
  if (a == NULL) {
    perror("Cannot make leaf");
    exit(1);
  }
  a->type = LEAF;
  a->left = (NODE *)l;
  a->right = NULL;
  return a;
}

int node_is_null(NODE *n) { return n == NULL; }
