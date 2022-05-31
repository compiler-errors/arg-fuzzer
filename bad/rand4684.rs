
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4684(_: S5, _: S6, _: S4, _: S1) {}
        
        fn test4684() { foo4684(S2, S3, S4, S5, S7); }
    