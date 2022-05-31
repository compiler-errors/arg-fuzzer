
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10034(_: S2, _: S3, _: S4, _: S6, _: S7) {}
        
        fn test10034() { foo10034(S1, S2, S5, S6, S7, S8); }
    