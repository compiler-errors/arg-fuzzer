
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5774(_: S1, _: S2, _: S8, _: S1, _: S5, _: S1) {}
        
        fn test5774() { foo5774(S5, S4, S3, S2, S8, S6, S7, S1); }
    