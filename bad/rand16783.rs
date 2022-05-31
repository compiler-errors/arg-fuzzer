
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16783(_: S2, _: S7, _: S1, _: S3) {}
        
        fn test16783() { foo16783(S7, S2, S6, S1, S5, S0, S2); }
    