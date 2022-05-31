
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9846(_: S2, _: S4, _: S6, _: S7, _: S8) {}
        
        fn test9846() { foo9846(S1, S5, S3, S2, S0, S4, S0); }
    