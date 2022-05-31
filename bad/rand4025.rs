
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4025(_: S5, _: S7, _: S8) {}
        
        fn test4025() { foo4025(S6, S5, S3, S2, S3); }
    