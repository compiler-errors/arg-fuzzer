
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16211(_: S3, _: S5, _: S6, _: S7, _: S8) {}
        
        fn test16211() { foo16211(S2, S1, S3, S7, S7, S4, S2); }
    