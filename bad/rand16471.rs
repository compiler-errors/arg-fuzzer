
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16471(_: S4, _: S7, _: S1) {}
        
        fn test16471() { foo16471(S3, S4, S6, S5); }
    