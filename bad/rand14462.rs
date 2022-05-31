
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14462(_: S1, _: S5, _: S6, _: S8) {}
        
        fn test14462() { foo14462(S2, S4, S5, S6, S7); }
    