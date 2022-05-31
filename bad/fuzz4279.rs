
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4279(_: S2, _: S3, _: S5, _: S6, _: S8) {}
        
        fn test4279() { foo4279(S2, S1, S2, S5, S6, S7, S4, S7); }
    