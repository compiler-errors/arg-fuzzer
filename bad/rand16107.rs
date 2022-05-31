
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16107(_: S6, _: S5, _: S1) {}
        
        fn test16107() { foo16107(S5, S2, S7, S6, S3); }
    