
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5144(_: S1, _: S5, _: S8) {}
        
        fn test5144() { foo5144(S1, S7, S5, S2); }
    